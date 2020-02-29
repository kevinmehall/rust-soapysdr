extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    let lib = match pkg_config::Config::new().atleast_version("0.6.0").probe("SoapySDR") {
        Err(e) => panic!("Couldn't find SoapySDR: {}", e),
        Ok(lib) => lib,
    };

    let mut bindings = bindgen::Builder::default()
        .trust_clang_mangling(false)
        .header("wrapper.h");
    
    for inc in lib.include_paths {
        let inc_str = inc.to_str().expect("PathBuf to string conversion problem");
        bindings = bindings.clang_arg("-I".to_owned() + inc_str);
    }
    
    let bindings = bindings.generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
