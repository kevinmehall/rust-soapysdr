extern crate bindgen;
extern crate cc;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    let lib = match pkg_config::Config::new().atleast_version("0.6.0").probe("SoapySDR") {
        Err(e) => panic!("Couldn't find SoapySDR: {}", e),
        Ok(lib) => lib,
    };

    let mut bindgen_builder = bindgen::Builder::default()
        .trust_clang_mangling(false)
        .header("wrapper.h");

    let mut cc_builder = cc::Build::new();
    cc_builder.file("wrapper.c");

    for inc in lib.include_paths {
        let inc_str = &inc.to_str().expect("PathBuf to string conversion problem");
        bindgen_builder = bindgen_builder.clang_arg("-I".to_owned() + inc_str);

        cc_builder.include(&inc);
    }

    let bindings = bindgen_builder.generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc_builder.compile("soapysdr-sys-wrappers");
}
