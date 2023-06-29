//! [SoapySDR](https://github.com/pothosware/SoapySDR/wiki) provides a hardware abstraction layer
//! for transmitting and receiving with many software defined radio devices.
//!
//!

extern crate num_complex;
extern crate soapysdr_sys;
#[cfg(feature = "log")]
#[macro_use]
extern crate log;

mod args;
pub use args::{Args, ArgsIterator};

mod arginfo;
pub use arginfo::ArgInfo;

mod device;
pub use device::{enumerate, Device, Direction, Error, ErrorCode, Range, RxStream, TxStream};

mod format;
pub use format::{Format, StreamSample};

/// Configures SoapySDR to log to the Rust `log` facility.
///
/// With `env_logger`, use e.g `RUST_LOG=soapysdr=info` to control the log level.
#[cfg(feature = "log")]
pub fn configure_logging() {
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
