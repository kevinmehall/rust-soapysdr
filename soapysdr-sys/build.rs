extern crate bindgen;
extern crate cc;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn probe_pkg_config() -> Option<Vec<PathBuf>> {
    match pkg_config::Config::new()
        .atleast_version("0.6.0")
        .probe("SoapySDR")
    {
        Err(e) => {
            eprintln!("pkg_config: {}", e);
            None
        }
        Ok(lib) => Some(lib.include_paths),
    }
}

// Find PothosSDR in paths in windows
fn probe_pothos_sdr() -> Option<Vec<PathBuf>> {
    #[cfg(windows)]
    {
        let lib = "SoapySDR";
        let dll = lib.to_owned() + ".dll";
        let paths = env::var_os("PATH")?;
        for dir in env::split_paths(&paths) {
            let dll_path = dir.join(&dll);
            let inc_path = dir.join("../include");
            let lib_path = dir.join("../lib");
            if dll_path.is_file() && inc_path.is_dir() {
                // Add lib directory for MSVC
                if lib_path.is_dir() {
                    println!("cargo:rustc-link-search={}", lib_path.to_str().unwrap());
                }
                println!("cargo:rustc-link-search={}", dir.to_str().unwrap());
                println!("cargo:rustc-link-lib={}", lib);
                return Some(vec![inc_path]);
            }
        }
    }
    None
}

fn main() {
    let include_paths = probe_pkg_config()
        .or_else(|| probe_pothos_sdr())
        .expect("Couldn't find SoapySDR");

    let mut bindgen_builder = bindgen::Builder::default()
        .trust_clang_mangling(false)
        .size_t_is_usize(true)
        .header("wrapper.h");

    let mut cc_builder = cc::Build::new();
    cc_builder.file("wrapper.c");

    for inc in include_paths {
        let inc_str = &inc.to_str().expect("PathBuf to string conversion problem");
        bindgen_builder = bindgen_builder.clang_arg("-I".to_owned() + inc_str);

        cc_builder.include(&inc);
    }

    // Wrapped by _rust_wrapper_SoapySDRDevice_setupStream for 0.7 -> 0.8 compatibility
    bindgen_builder = bindgen_builder.blocklist_function("SoapySDRDevice_setupStream");

    let bindings = bindgen_builder.generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc_builder.compile("soapysdr-sys-wrappers");
}
