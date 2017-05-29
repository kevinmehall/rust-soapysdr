extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    if let Err(e) = pkg_config::Config::new().atleast_version("0.6.0").probe("SoapySDR") {
        panic!("Couldn't find SoapySDR: {}", e);
    }

    let bindings = bindgen::Builder::default()
        .no_unstable_rust()
        .trust_clang_mangling(false)
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
