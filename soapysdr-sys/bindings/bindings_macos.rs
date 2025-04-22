// macos tests + data structures

#[doc = "! Possible data types for argument info"]
pub type SoapySDRArgInfoType = ::std::os::raw::c_uint;

#[doc = " The available priority levels for log messages.\n\n The default log level threshold is SOAPY_SDR_INFO.\n Log messages with lower priorities are dropped.\n\n The default threshold can be set via the\n SOAPY_SDR_LOG_LEVEL environment variable.\n Set SOAPY_SDR_LOG_LEVEL to the string value:\n \"WARNING\", \"ERROR\", \"DEBUG\", etc...\n or set it to the equivalent integer value."]
pub type SoapySDRLogLevel = ::std::os::raw::c_uint;

extern "C" {
    #[doc = " Send a message to the registered logger.\n \\param logLevel a possible logging level\n \\param format a printf style format string\n \\param argList an argument list for the formatter"]
    pub fn SoapySDR_vlogf(
        logLevel: SoapySDRLogLevel,
        format: *const ::std::os::raw::c_char,
        argList: va_list,
    );
}

pub type va_list = __builtin_va_list;
pub type __builtin_va_list = *mut ::std::os::raw::c_char;
