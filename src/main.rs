// main.rs
// :PROPERTIES:
// :header-args: :tangle src/main.rs
// :END:

// [[file:~/Workspace/Programming/rust-libs/lbfgs/lbfgs.note::*main.rs][main.rs:1]]
use liblbfgs::*;

const N: usize = 100;
fn main() {
    // Initialize the variables
    let mut x = [0.0 as lbfgsfloatval_t; N];
    for i in (0..N).step_by(2) {
        x[i] = -1.2;
        x[i+1] = 1.0;
    }

    let mut l = LBFGS::default();
    let fx = l.run(&mut x, evaluate_default).expect("lbfgs run");
    println!("  fx = {:}, x[0] = {}, x[1] = {}\n", fx, x[0], x[1]);
}
// main.rs:1 ends here
