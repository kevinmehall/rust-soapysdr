use std::env;
use std::env::consts;

fn probe_env_var() -> bool {
    if let Some(paths) = env::var_os("SOAPY_SDR_ROOT").or_else(|| env::var_os("SoapySDR_DIR")) {
        for path in env::split_paths(&paths) {
            let dylib_name = format!("{}SoapySDR{}", consts::DLL_PREFIX, consts::DLL_SUFFIX);
            let lib_path = path.join("./lib");

            if lib_path.is_dir() && lib_path.join(dylib_name).exists() {
                println!("cargo:rustc-link-search={}", lib_path.to_str().unwrap());
                println!("cargo:rustc-link-lib=SoapySDR");
                return true;
            }
        }
    }
    false
}

fn probe_pkg_config() -> bool {
    match pkg_config::Config::new()
        .atleast_version("0.8.0")
        .probe("SoapySDR")
    {
        #[cfg(not(windows))]
        // windows users likely don't have pkg_config installed, so
        // this would be a confusing error message
        Err(e) => {
            eprintln!("pkg_config: {}", e);
            false
        }
        #[cfg(windows)]
        Err(_) => false,
        Ok(_) => true,
    }
}

// Find PothosSDR in paths in windows
fn probe_pothos_sdr() -> bool {
    #[cfg(target_os = "windows")]
    {
        let lib = "SoapySDR";
        let dll = lib.to_owned() + ".dll";
        let Some(paths) = env::var_os("PATH") else {
            return false;
        };
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
                return true;
            }
        }
    }
    false
}

fn panic_help_message_soapysdr() -> ! {
    #[cfg(target_os = "windows")]
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
    #[cfg(target_os = "macos")]
    {
        const MSG: &str = "


        ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        SoapySDR (PothosSDR) and pkg-config are required but could not be found.
        Please install them with Homebrew (https://brew.sh)


                brew install pkg-config
                brew tap pothosware/homebrew-pothos
                brew update

                followed by the soapy package for your radio,
                e.g.
                  * brew install soapyrtlsdr
                  * brew install soapyhackrf
                  * brew install soapybladerf
                  * etc...


        and then try again.


        Visit https://github.com/kevinmehall/rust-soapysdr for more information.
        ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


        ";
        panic!("{}", MSG);
    }
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        panic!(
            "SoapySDR is required but could not be found. Please install SoapySDR and try again."
        );
    }
}

fn main() {
    if !(probe_env_var() || probe_pkg_config() || probe_pothos_sdr()) {
        panic_help_message_soapysdr();
    }
}
