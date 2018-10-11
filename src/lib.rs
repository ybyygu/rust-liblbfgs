// base

// [[file:~/Workspace/Programming/rust-libs/lbfgs/lbfgs.note::*base][base:1]]
use std::ptr::null_mut;
use std::os::raw::{c_int, c_void};
use quicli::prelude::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// See http://www.chokkan.org/software/liblbfgs/structlbfgs__parameter__t.html for documentation.
pub type LBFGSParameter = lbfgs_parameter_t;

impl Default for LBFGSParameter {
    /// default LBFGS parameter
    fn default() -> Self {
        // LBFGSParameter {
        //     m: 6,
        //     epsilon: 1e-5,
        //     past: 0,
        //     delta: 1e-5,
        //     max_iterations: 0,
        //     linesearch: 0,
        //     max_linesearch: 40,
        //     min_step: 1e-20,
        //     max_step: 1e20,
        //     ftol: 1e-4,
        //     wolfe: 0.9,
        //     gtol: 0.9,
        //     xtol: 1.0e-16,
        //     orthantwise_c: 0.0,
        //     orthantwise_start: 0,
        //     orthantwise_end: -1,
        // }

        let mut param: lbfgs_parameter_t;
        unsafe {
            param = ::std::mem::uninitialized();
            lbfgs_parameter_init(&mut param);
        }
        param
    }
}
// base:1 ends here

// lbfgs

// [[file:~/Workspace/Programming/rust-libs/lbfgs/lbfgs.note::*lbfgs][lbfgs:1]]
// for evaluate variables
pub type EvaluateFn = fn(x: &[f64], gx: &mut [f64]) -> Result<f64>;

// for monitoring optimization progress
pub type ProgressFn = fn(prgr: &Progress) -> bool;

#[derive(Clone)]
#[repr(C)]
pub struct LBFGS {
    pub param: LBFGSParameter,
    progress: ProgressFn,
    evaluate: EvaluateFn,
}

impl Default for LBFGS {
    fn default() -> Self {
        LBFGS {
            param   : LBFGSParameter::default(),
            evaluate: evaluate_default,
            progress: progress_default,
        }
    }
}

impl LBFGS {
    /// Start the L-BFGS optimization; this will invoke the callback functions
    /// evaluate() and progress() when necessary.
    ///
    /// # Parameters
    /// - arr_x  : The array of variables, which will be updated during optimization.
    /// - eval_fn: A closure to evaluate arr_x
    pub fn run(&mut self, arr_x: &mut [f64], eval_fn: EvaluateFn) -> Result<f64>
    {
        self.evaluate = eval_fn;

        // Cast LBFGS as a void pointer for passing it to lbfgs as the instance
        // parameter
        // let instance = &self.to_ptr();
        let instance = self as *const _ as *mut c_void;

        // call external lbfgs function
        let n = arr_x.len();
        let mut fx = 0.0;
        let ret = unsafe {
            lbfgs(n as c_int,
                  arr_x.as_mut_ptr(),
                  &mut fx,
                  Some(evaluate_wrapper),
                  Some(progress_wrapper),
                  instance,
                  &mut self.param
            )
        };

        if ret == 0 {
            Ok(fx)
        } else {
            bail!("lbfgs failed with status code = {}", ret);
        }
    }
}
// lbfgs:1 ends here

// callback: progress

// [[file:~/Workspace/Programming/rust-libs/lbfgs/lbfgs.note::*callback:%20progress][callback: progress:1]]
type Cancel = bool;

impl LBFGS {
    /// Assign a callback function for monitoring optimization progress
    pub fn set_progress_monitor(&mut self, prgr_fn: ProgressFn) {
        self.progress = prgr_fn;
    }
}

/// Store optimization progress data
pub struct Progress<'a> {
    /// The current values of variables
    arr_x: &'a [f64],
    /// The current gradient values of variables.
    grd_x: &'a [f64],
    /// The current value of the objective function.
    fx: f64,
    /// The Euclidean norm of the variables
    xnorm: f64,
    /// The Euclidean norm of the gradients.
    gnorm: f64,
    /// The line-search step used for this iteration.
    step: f64,
    /// The iteration count.
    niter: usize,
    /// The number of evaluations called for this iteration.
    ncall: usize
}

/// default progress monitor
pub fn progress_default(prgr: &Progress) -> Cancel {
    let x = &prgr.arr_x;

    println!("Iteration {}:", &prgr.niter);
    println!("  fx = {}, x[0] = {}, x[1] = {}", &prgr.fx, x[0], x[1]);
    println!("  xnorm = {}, gnorm = {}, step = {}", &prgr.xnorm, &prgr.gnorm, &prgr.step);
    println!("");

    false
}

// for converting rust instance to a C progress callback function
extern fn progress_wrapper(
    instance : *mut c_void,
    x        : *const lbfgsfloatval_t,
    g        : *const lbfgsfloatval_t,
    fx       : lbfgsfloatval_t,
    xnorm    : lbfgsfloatval_t,
    gnorm    : lbfgsfloatval_t,
    step     : lbfgsfloatval_t,
    n        : c_int,
    k        : c_int,
    ls       : c_int) -> c_int
{
    let n = n as usize;
    // convert pointer to native data type
    let arr_x = unsafe {
        ::std::slice::from_raw_parts(x, n)
    };

    // convert pointer to native data type
    let gx = unsafe {
        ::std::slice::from_raw_parts(g, n)
    };

    // cast as LBFGS instance
    let ptr_lbfgs = instance as *mut LBFGS;
    unsafe {
        let prgr = Progress {
            arr_x,
            fx,
            xnorm,
            gnorm,
            step,
            grd_x: gx,
            niter: k as usize,
            ncall: ls as usize
        };

        let to_cancel = (*ptr_lbfgs).progress;
        if to_cancel(&prgr) {
            return 1 as c_int
        } else {
            0 as c_int
        }
    }
}
// callback: progress:1 ends here

// callback: evalulate

// [[file:~/Workspace/Programming/rust-libs/lbfgs/lbfgs.note::*callback:%20evalulate][callback: evalulate:1]]
// # Parameters
// - fx: evaluated value
// - gx: gradients of arr_x
extern fn evaluate_wrapper(instance: *mut c_void,
                           x: *const lbfgsfloatval_t,
                           g: *mut lbfgsfloatval_t,
                           n: c_int,
                           step: lbfgsfloatval_t) -> lbfgsfloatval_t
{
    let n = n as usize;
    // convert pointer to native data type
    let arr_x = unsafe {
        ::std::slice::from_raw_parts(x, n)
    };

    // convert pointer to native data type
    let mut gx = unsafe {
        ::std::slice::from_raw_parts_mut(g, n)
    };

    // cast as Rust instance
    let ptr_lbfgs = instance as *mut LBFGS;
    let feval = unsafe {
        (*ptr_lbfgs).evaluate
    };
    let fx = feval(&arr_x, &mut gx).expect("eval instance");
    fx
}

// default evaluator adopted from liblbfgs sample.c
// # Parameters
// - gx: gradients of arr_x
// - fx: evaluated value
pub fn evaluate_default(arr_x: &[f64], gx: &mut [f64]) -> Result<f64> {
    let n = arr_x.len();
    assert_eq!(n, gx.len(), "slice length diff in lbfgs evaluate");

    let mut fx: lbfgsfloatval_t = 0.0;
    for i in (0..n).step_by(2) {
        let t1: lbfgsfloatval_t = 1.0 - arr_x[i];
        let t2: lbfgsfloatval_t = 10.0 * (arr_x[i+1] - arr_x[i] * arr_x[i]);
        gx[i+1] = 20.0 * t2;
        gx[i] = -2.0 * (arr_x[i] * gx[i+1] + t1);
        fx += t1 * t1 + t2 * t2;
    }

    Ok(fx)
}
// callback: evalulate:1 ends here
