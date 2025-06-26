// linux tests + data structures

#[doc = "! Possible data types for argument info"]
pub type SoapySDRArgInfoType = ::std::os::raw::c_uint;

#[doc = " The available priority levels for log messages.\n\n The default log level threshold is SOAPY_SDR_INFO.\n Log messages with lower priorities are dropped.\n\n The default threshold can be set via the\n SOAPY_SDR_LOG_LEVEL environment variable.\n Set SOAPY_SDR_LOG_LEVEL to the string value:\n \"WARNING\", \"ERROR\", \"DEBUG\", etc...\n or set it to the equivalent integer value."]
pub type SoapySDRLogLevel = ::std::os::raw::c_uint;

extern "C" {
    #[doc = " Send a message to the registered logger.\n \\param logLevel a possible logging level\n \\param format a printf style format string\n \\param argList an argument list for the formatter"]
    pub fn SoapySDR_vlogf(
        logLevel: SoapySDRLogLevel,
        format: *const ::std::os::raw::c_char,
        argList: *mut __va_list_tag,
    );
}

pub type va_list = __builtin_va_list;
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout___va_list_tag() {
    const UNINIT: ::std::mem::MaybeUninit<__va_list_tag> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__va_list_tag>(),
        24usize,
        concat!("Size of: ", stringify!(__va_list_tag))
    );
    assert_eq!(
        ::std::mem::align_of::<__va_list_tag>(),
        8usize,
        concat!("Alignment of ", stringify!(__va_list_tag))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gp_offset) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(gp_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fp_offset) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(fp_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).overflow_arg_area) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(overflow_arg_area)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).reg_save_area) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(reg_save_area)
        )
    );
}

// preserve any bindgen tests for all platforms out of an abundance of caution
// these get run as part of `cargo test`

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}

#[test]
fn bindgen_test_layout_max_align_t() {
    const UNINIT: ::std::mem::MaybeUninit<max_align_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<max_align_t>(),
        32usize,
        concat!("Size of: ", stringify!(max_align_t))
    );
    assert_eq!(
        ::std::mem::align_of::<max_align_t>(),
        16usize,
        concat!("Alignment of ", stringify!(max_align_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__clang_max_align_nonce1) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__clang_max_align_nonce2) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce2)
        )
    );
}

