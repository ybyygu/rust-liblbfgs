
# lbfgs

This is a Rust wrapper around Naoaki Okazaki (chokkan)&rsquo;s [liblbfgs](http://chokkan.org/software/liblbfgs/) library of
quasi-Newton optimization routines (limited memory BFGS and OWL-QN).


# Usage

    // 0. Import the lib
    use liblbfgs::LBFGS;
    
    // 1. Initialize data
    let mut x = [0.0 as f64; N];
    for i in (0..N).step_by(2) {
        x[i] = -1.2;
        x[i+1] = 1.0;
    }
    
    // 2. Define evaluator callback
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
    
    // 3. Carry out LBFGS optimization
    let mut lbfgs = LBFGS::default();
    let fx = lbfgs.run(&mut x, evaluate).expect("lbfgs run");

Run the example:

    cargo run --example sample

