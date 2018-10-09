// build.rs
// :PROPERTIES:
// :header-args: :tangle build.rs
// :END:

// [[file:~/Workspace/Programming/rust-libs/rust-liblbfgs/lbfgs.note::*build.rs][build.rs:1]]
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=lbfgs");

    let bindings = bindgen::Builder::default()
        .header("liblbfgs/include/lbfgs.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
// build.rs:1 ends here
