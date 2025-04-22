use std::env;
use std::env::consts;
use std::path::PathBuf;

fn probe_env_var() -> Option<Vec<PathBuf>> {
    let paths = env::var_os("SOAPY_SDR_ROOT").or_else(|| env::var_os("SoapySDR_DIR"))?;
    for path in env::split_paths(&paths) {
        let dylib_name = format!("{}SoapySDR{}", consts::DLL_PREFIX, consts::DLL_SUFFIX);
        let inc_path = path.join("./include");
        let lib_path = path.join("./lib");

        if lib_path.is_dir() && inc_path.is_dir() && lib_path.join(dylib_name).exists() {
            println!("cargo:rustc-link-search={}", lib_path.to_str().unwrap());
            println!("cargo:rustc-link-lib=SoapySDR");

            return Some(vec![inc_path]);
        }
    }
    None
}

fn probe_pkg_config() -> Option<Vec<PathBuf>> {
    match pkg_config::Config::new()
        .atleast_version("0.6.0")
        .probe("SoapySDR")
    {
        #[cfg(not(windows))]
        // windows users likely don't have pkg_config installed, so
        // this would be a confusing error message
        Err(e) => {
            eprintln!("pkg_config: {}", e);
            None
        }
        #[cfg(windows)]
        Err(_) => {
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

fn panic_help_message_soapysdr() -> ! {
    #[cfg(windows)]
    {
        const MSG: &str = "


        ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        SoapySDR (PothosSDR) is required but could not be found.
        Please install PothosSDR from


                https://downloads.myriadrf.org/builds/PothosSDR/

                and select 'Add PothosSDR to the system PATH for all users'
                or 'Add PothosSDR to the system PATH for the current user'

        and then try again in a new command prompt.


        Visit https://github.com/kevinmehall/rust-soapysdr for more information.
        ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


        ";
        panic!("{}", MSG);
    }
    #[cfg(not(windows))]
    {
        panic!("SoapySDR is required but could not be found. Please install SoapySDR and try again.");
    }
}

fn panic_help_message_libclang() -> ! {
    #[cfg(windows)]
    {
        let msg = "


        ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        libclang (llvm) is required but could not be found.
        Please install llvm from


                https://releases.llvm.org/download.html

                and select 'Add LLVM to the system PATH for all users'
                or 'Add LLVM to the system PATH for the current user'


        and then try again in a new command prompt.


        Visit https://rust-lang.github.io/rust-bindgen/requirements.html for more information.
        ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


        ";
        panic!("{}", msg);
    }
    #[cfg(not(windows))]
    {
        panic!("libclang is required but could not be found. Please install libclang and try again.");
    }
}

fn main() {
    let include_paths = probe_env_var()
        .or_else(probe_pkg_config)
        .or_else(probe_pothos_sdr)
        .unwrap_or_else(|| panic_help_message_soapysdr());

    if let Err(_) = std::panic::catch_unwind(|| bindgen::clang_version()) {
        panic_help_message_libclang();
    }

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
