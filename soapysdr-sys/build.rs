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
            eprintln!("pkg_config: {e}");
            None
        }
        #[cfg(windows)]
        Err(_) => None,
        Ok(lib) => {
            for link_path in lib.link_paths {
                println!("cargo:rustc-link-search={}", link_path.display());
            }
            for lib in lib.libs {
                println!("cargo:rustc-link-lib={lib}");
            }
            Some(lib.include_paths)
        }
    }
}

// Find PothosSDR in paths in windows
fn probe_pothos_sdr() -> Option<Vec<PathBuf>> {
    #[cfg(target_os = "windows")]
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

fn panic_help_message_libclang() -> ! {
    #[cfg(target_os = "windows")]
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
    #[cfg(not(target_os = "windows"))]
    {
        panic!(
            "libclang is required but could not be found. Please install libclang and try again."
        );
    }
}

fn build_bundled_soapysdr(build_static: bool) -> Vec<PathBuf> {
    let revision_to_build =
        std::env::var("SOAPY_SDR_TAG").unwrap_or_else(|_| "soapy-sdr-0.8.1".to_string());
    let revision_to_build = revision_to_build.as_str();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let soapysdr_dir = out_dir.join("SoapySDR");

    // Keep a stamp file of the version we cloned to check if we need to clone from fresh if the
    // above revision_to_build string changes
    let revision_file = out_dir.join(".soapysdr_revision");

    let needs_clone = if let Ok(cached_revision) = std::fs::read_to_string(&revision_file) {
        cached_revision.trim() != revision_to_build
    } else {
        true
    };

    if needs_clone {
        if let Err(e) = std::fs::remove_dir_all(&soapysdr_dir) {
            assert_eq!(
                e.kind(),
                std::io::ErrorKind::NotFound,
                "Failed to remove old SoapySDR directory: {e}"
            );
        }

        let status = std::process::Command::new("git")
            .arg("clone")
            .arg("--depth")
            .arg("1")
            .arg("--branch")
            .arg(revision_to_build)
            .arg("https://github.com/pothosware/SoapySDR.git")
            .arg(&soapysdr_dir)
            .status()
            .expect("Failed to execute git clone");

        assert!(
            status.success(),
            "Failed to clone SoapySDR: git exited with status {status}"
        );

        std::fs::write(&revision_file, revision_to_build).expect("Failed to write revision cache");
    }

    if build_static {
        let lib_cmakelists = soapysdr_dir.join("lib/CMakeLists.txt");
        let lib_cmakelists_content =
            std::fs::read_to_string(&lib_cmakelists).expect("Failed to read lib/CMakeLists.txt");

        // Patch the CMakeLists.txt to build static instead of shared as unfortunately it looks like the upstream
        // project forces shared libraries instead of respecting BUILD_SHARED_LIBS.
        let patched_content = lib_cmakelists_content
            .replace("add_library(SoapySDR SHARED", "add_library(SoapySDR STATIC");

        std::fs::write(&lib_cmakelists, patched_content)
            .expect("Failed to write patched lib/CMakeLists.txt");

        // Patch Config.h to undefine SOAPY_SDR_DLL for static builds
        let config_h = soapysdr_dir.join("include/SoapySDR/Config.h");
        let config_h_content = std::fs::read_to_string(&config_h).expect("Failed to read Config.h");

        let patched_config = config_h_content.replace(
            "#define SOAPY_SDR_DLL //always building a DLL",
            "// #define SOAPY_SDR_DLL //always building a DLL (disabled for static build)",
        );

        std::fs::write(&config_h, patched_config).expect("Failed to write patched Config.h");
    }

    println!("cargo:rerun-if-env-changed=SOAPY_SDR_ROOT");

    let install_prefix = out_dir.join("soapysdr-install");
    cmake::Config::new(&soapysdr_dir)
        // Compatibility if building with CMake 4 since the upstream project doesn't specify this.
        .define("CMAKE_POLICY_VERSION_MINIMUM", "3.5")
        .define("CMAKE_INSTALL_PREFIX", &install_prefix)
        // If SOAPY_SDR_ROOT isn't specified as an environment variable in the build,
        // default to telling SoapySDR it's being installed to /usr. Failure to properly configure
        // this screws up module discovery.
        .define(
            "SOAPY_SDR_ROOT",
            std::env::var("SOAPY_SDR_ROOT").unwrap_or_else(|_| "/usr".to_string()),
        )
        // Disable a bunch of SoapySDR features except for the ones needed to build the library.
        .define("ENABLE_LIBRARY", "ON")
        .define("ENABLE_APPS", "OFF")
        .define("ENABLE_LDOC", "OFF")
        .define("ENABLE_CSHARP", "OFF")
        .define("ENABLE_DOCS", "OFF")
        .define("ENABLE_PYTHON2", "OFF")
        .define("ENABLE_PYTHON3", "OFF")
        .define("ENABLE_TESTS", "OFF")
        .define("ENABLE_LUAJIT", "OFF")
        .build_target("install")
        .build();

    let lib_kind = if build_static { "static" } else { "dylib" };

    println!(
        "cargo:rustc-link-search=native={}/lib",
        install_prefix.display()
    );
    println!("cargo:rustc-link-lib={lib_kind}=SoapySDR");

    // Any libraries SoapySDR needs to be linked against need to be specified here since static libraries don't
    // carry that information for you.
    if cfg!(unix) && build_static {
        // Detect which C++ standard library to use based on the compiler
        let cpp_lib = if cfg!(target_env = "musl") {
            // musl-based systems typically use libstdc++
            "stdc++"
        } else if std::env::var("RUSTFLAGS")
            .ok()
            .is_some_and(|f| f.contains("libc++"))
            || std::env::var("RUSTFLAGS")
                .ok()
                .is_some_and(|f| f.contains("stdlib=libc++"))
        {
            // User explicitly requested libc++
            "c++"
        } else if cfg!(target_os = "macos") {
            // macOS defaults to libc++
            "c++"
        } else {
            // Linux and others typically use libstdc++
            "stdc++"
        };
        println!("cargo:rustc-link-lib={cpp_lib}");
        println!("cargo:rustc-link-lib=pthread");
    }

    vec![install_prefix.join("include")]
}

fn main() {
    println!("cargo:rerun-if-env-changed=BUILD_SOAPYSDR_STATIC");

    let include_paths = if cfg!(feature = "bundled") {
        build_bundled_soapysdr(std::env::var("BUILD_SOAPYSDR_STATIC").is_ok_and(|v| v == "1"))
    } else {
        probe_env_var()
            .or_else(probe_pkg_config)
            .or_else(probe_pothos_sdr)
            .unwrap_or_else(|| panic_help_message_soapysdr())
    };

    if std::panic::catch_unwind(bindgen::clang_version).is_err() {
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

    let bindings = bindgen_builder
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc_builder.compile("soapysdr-sys-wrappers");
}
