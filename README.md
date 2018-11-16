
# lbfgs

This is a Rusty wrapper built around the [libLBFGS](http://chokkan.org/software/liblbfgs/) library of quasi-Newton
optimization routines (limited memory BFGS and OWL-QN) by Naoaki Okazaki.

For further information, please refer to the [libLBFGS](http://www.chokkan.org/software/liblbfgs/structlbfgs__parameter__t.html) page for tunable parameters.


# Usage

    // 0. Import the lib
    use liblbfgs::{LBFGS, Progress};
    
    // 1. Initialize data
    let mut x = [0.0 as f64; N];
    for i in (0..N).step_by(2) {
        x[i] = -1.2;
        x[i+1] = 1.0;
    }
    
    // 2. Define evaluator callback
    let evaluate= |arr_x: &[f64], gx: &mut [f64]| {
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
    };
    
    // 3. Carry out LBFGS optimization
    let mut lbfgs = LBFGS::default();
    let fx = lbfgs.run(&mut x, evaluate, |_: &Progress| false).expect("lbfgs run");

The callback functions are native Rust FnMut closures, possible to
capture/change variables in the environment.

Full codes with comments are available in examples/sample.rs.

Run the example:

    cargo run --example sample

