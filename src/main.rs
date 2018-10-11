// main.rs
// :PROPERTIES:
// :header-args: :tangle src/main.rs
// :END:

// [[file:~/Workspace/Programming/rust-libs/lbfgs/lbfgs.note::*main.rs][main.rs:1]]
use std::ptr::null_mut;
use std::os::raw::{c_int, c_void};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

const N: usize = 100;
extern "C" fn progress(
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
    let x = unsafe {
        ::std::slice::from_raw_parts(x, n)
    };

    // convert pointer to native data type
    let g = unsafe {
        ::std::slice::from_raw_parts(g, n)
    };

    println!("Iteration {}:", k);
    println!("  fx = {}, x[0] = {}, x[1] = {}", fx, x[0], x[1]);
    println!("  xnorm = {}, gnorm = {}, step = {}", xnorm, gnorm, step);
    println!("");
    return 0;
}

extern "C" fn evaluate(instance: *mut c_void,
                       x: *const lbfgsfloatval_t,
                       g: *mut lbfgsfloatval_t,
                       n: c_int,
                       step: lbfgsfloatval_t) -> lbfgsfloatval_t
{
    let n = n as usize;
    // convert pointer to native data type
    let x = unsafe {
        ::std::slice::from_raw_parts(x, n)
    };

    // convert pointer to native data type
    let mut g = unsafe {
        ::std::slice::from_raw_parts_mut(g, n)
    };

    let mut fx: lbfgsfloatval_t = 0.0;
    for i in (0..n).step_by(2) {
        let t1: lbfgsfloatval_t = 1.0 - x[i];
        let t2: lbfgsfloatval_t = 10.0 * (x[i+1] - x[i] * x[i]);
        g[i+1] = 20.0 * t2;
        g[i] = -2.0 * (x[i] * g[i+1] + t1);
        fx += t1 * t1 + t2 * t2;
    }

    fx
}

fn main() {
    // Initialize the variables
    let mut x = [0.0 as lbfgsfloatval_t; N];
    for i in (0..N).step_by(2) {
        x[i] = -1.2;
        x[i+1] = 1.0;
    }

    // Start the L-BFGS optimization; this will invoke the callback functions
    // evaluate() and progress() when necessary.
    let mut fx: lbfgsfloatval_t = 0.0;
    unsafe {
        for i in (0..N).step_by(2) {
            x[i] = -1.2;
            x[i+1] = 1.0;
        }

        // initial parameters
        let mut param: lbfgs_parameter_t;
        param = ::std::mem::uninitialized();
        lbfgs_parameter_init(&mut param);

        let ret = lbfgs(N as c_int,
              x.as_mut_ptr(),
              &mut fx,
              Some(evaluate),
              Some(progress),
              null_mut(),
              &mut param
        );

        // Report the result.
        println!("L-BFGS optimization terminated with status code = {:?}", ret);
        println!("  fx = {}, x[0] = {}, x[1] = {}\n", fx, x[0], x[1]);
    };
}
// main.rs:1 ends here
