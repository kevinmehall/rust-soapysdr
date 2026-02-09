//! [SoapySDR](https://github.com/pothosware/SoapySDR/wiki) provides a hardware abstraction layer
//! for transmitting and receiving with many software defined radio devices.
//!
//!

mod args;
pub use args::{Args, ArgsIterator};

mod arginfo;
pub use arginfo::ArgInfo;

mod device;
pub use device::{enumerate, Device, Direction, Error, ErrorCode, Range, RxStream, TxStream};

mod format;
pub use format::{Format, StreamSample};

unsafe fn to_string_vec(
    f: unsafe extern "C" fn(length: *mut usize) -> *mut *mut std::ffi::c_char,
) -> Vec<String> {
    let mut length = 0;
    let array_of_cstring = unsafe { f(&mut length) };
    if array_of_cstring.is_null() {
        return vec![];
    }

    unsafe { std::slice::from_raw_parts(array_of_cstring, length) }
        .iter()
        .filter_map(|cstr| {
            unsafe { std::ffi::CStr::from_ptr(*cstr) }
                .to_str()
                .ok()
                .map(str::to_string)
        })
        .collect()
}

/// List the search paths for modules.
pub fn list_search_paths() -> Vec<String> {
    unsafe { to_string_vec(soapysdr_sys::SoapySDR_listSearchPaths) }
}

/// List the modules that have been found. The search path can be adjusted via the `SOAPY_SDR_PLUGIN_PATH` environment
/// variable if it's not found.
pub fn list_modules() -> Vec<String> {
    unsafe { to_string_vec(soapysdr_sys::SoapySDR_listModules) }
}

/// Load SoapySDR modules.
pub fn load_modules() {
    unsafe { soapysdr_sys::SoapySDR_loadModules() }
}

/// Unload SoapySDR modules.
pub fn unload_modules() {
    unsafe { soapysdr_sys::SoapySDR_unloadModules() }
}

/// Configures SoapySDR to log to the Rust `log` facility.
///
/// With `env_logger`, use e.g `RUST_LOG=soapysdr=info` to control the log level.
#[cfg(feature = "log")]
pub fn configure_logging() {
    use log::log;
    use log::Level;
    use soapysdr_sys::*;
    use std::ffi::CStr;
    use std::os::raw::c_char;

    extern "C" fn soapy_log(level: SoapySDRLogLevel, message: *const c_char) {
        #![allow(non_upper_case_globals)]
        let level = match level {
            SoapySDRLogLevel_SOAPY_SDR_FATAL => Level::Error,
            SoapySDRLogLevel_SOAPY_SDR_CRITICAL => Level::Error,
            SoapySDRLogLevel_SOAPY_SDR_ERROR => Level::Error,
            SoapySDRLogLevel_SOAPY_SDR_WARNING => Level::Warn,
            SoapySDRLogLevel_SOAPY_SDR_NOTICE => Level::Info,
            SoapySDRLogLevel_SOAPY_SDR_INFO => Level::Info,
            SoapySDRLogLevel_SOAPY_SDR_DEBUG => Level::Debug,
            SoapySDRLogLevel_SOAPY_SDR_TRACE => Level::Trace,
            SoapySDRLogLevel_SOAPY_SDR_SSI => Level::Info, // Streaming status indicators such as "U" (underflow) and "O" (overflow).
            _ => Level::Error,
        };

        let msg = unsafe { CStr::from_ptr(message) };
        log!(
            level,
            "{}",
            msg.to_string_lossy().trim_start_matches(&['\r', '\n'][..])
        );
    }

    unsafe {
        SoapySDR_registerLogHandler(Some(soapy_log));
    }
}
