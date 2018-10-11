// example
// :PROPERTIES:
// :header-args: :tangle examples/sample.rs
// :END:

// [[file:~/Workspace/Programming/rust-libs/lbfgs/lbfgs.note::*example][example:1]]
use liblbfgs::LBFGS;
use quicli::prelude::*;

// default evaluator adopted from liblbfgs sample.c
// # Parameters
// - gx: gradients of arr_x
// - fx: evaluated value
fn evaluate(arr_x: &[f64], gx: &mut [f64]) -> Result<f64> {
    let n = arr_x.len();

    let mut fx = 0.0;
    for i in (0..n).step_by(2) {
        let t1 = 1.0 - arr_x[i];
        let t2 = 10.0 * (arr_x[i+1] - arr_x[i] * arr_x[i]);
        gx[i+1] = 20.0 * t2;
        gx[i] = -2.0 * (arr_x[i] * gx[i+1] + t1);
        fx += t1 * t1 + t2 * t2;
    }

    Ok(fx)
}

fn main() {
    const N: usize = 100;

    // Initialize the variables
    let mut x = [0.0 as f64; N];
    for i in (0..N).step_by(2) {
        x[i] = -1.2;
        x[i+1] = 1.0;
    }

    let mut lbfgs = LBFGS::default();
    let fx = lbfgs.run(&mut x, evaluate).expect("lbfgs run");
    println!("  fx = {:}, x[0] = {}, x[1] = {}\n", fx, x[0], x[1]);
}
// example:1 ends here
