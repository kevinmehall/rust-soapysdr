//! [SoapySDR](https://github.com/pothosware/SoapySDR/wiki) provides a hardware abstraction layer
//! for transmitting and receiving with many software defined radio devices.
//!
//!

extern crate soapysdr_sys;
extern crate libc;
extern crate num_complex;
#[cfg(feature="log")] #[macro_use] extern crate log;

mod args;
pub use args::{Args, ArgsIterator};

mod arginfo;
pub use arginfo::ArgInfo;

mod device;
pub use device::{enumerate, Device, RxStream, TxStream, Error, ErrorCode, Direction, Range, Format, StreamSample};

/// Configures SoapySDR to log to the Rust `log` facility.
///
/// With `env_logger`, use e.g `RUST_LOG=soapysdr=info` to control the log level.
#[cfg(feature="log")]
pub fn configure_logging() {
    use log::LogLevel;
    use soapysdr_sys::*;
    use libc::c_char;
    use std::ffi::CStr;

    extern "C" fn soapy_log(level: SoapySDRLogLevel, message: *const c_char) {
        #![allow(non_upper_case_globals)]
        let level = match level {
                SoapySDRLogLevel_SOAPY_SDR_FATAL    => LogLevel::Error,
                SoapySDRLogLevel_SOAPY_SDR_CRITICAL => LogLevel::Error,
                SoapySDRLogLevel_SOAPY_SDR_ERROR    => LogLevel::Error,
                SoapySDRLogLevel_SOAPY_SDR_WARNING  => LogLevel::Warn,
                SoapySDRLogLevel_SOAPY_SDR_NOTICE   => LogLevel::Info,
                SoapySDRLogLevel_SOAPY_SDR_INFO     => LogLevel::Info,
                SoapySDRLogLevel_SOAPY_SDR_DEBUG    => LogLevel::Debug,
                SoapySDRLogLevel_SOAPY_SDR_TRACE    => LogLevel::Trace,
                SoapySDRLogLevel_SOAPY_SDR_SSI      => LogLevel::Info, // Streaming status indicators such as "U" (underflow) and "O" (overflow).
                _ => LogLevel::Error,
        };

        let msg = unsafe { CStr::from_ptr(message) };
        log!(level, "{}", msg.to_string_lossy().trim_left_matches(&['\r', '\n'][..]));
    }

    unsafe {
        SoapySDR_registerLogHandler(Some(soapy_log));
    }
}
