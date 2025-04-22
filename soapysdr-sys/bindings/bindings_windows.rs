// windows tests + data structures
#[doc = "! Possible data types for argument info"]
pub type SoapySDRArgInfoType = ::std::os::raw::c_int;

#[doc = " The available priority levels for log messages.\n\n The default log level threshold is SOAPY_SDR_INFO.\n Log messages with lower priorities are dropped.\n\n The default threshold can be set via the\n SOAPY_SDR_LOG_LEVEL environment variable.\n Set SOAPY_SDR_LOG_LEVEL to the string value:\n \"WARNING\", \"ERROR\", \"DEBUG\", etc...\n or set it to the equivalent integer value."]
pub type SoapySDRLogLevel = ::std::os::raw::c_int;

pub type va_list = *mut ::std::os::raw::c_char;

extern "C" {
    #[doc = " Send a message to the registered logger.\n \\param logLevel a possible logging level\n \\param format a printf style format string\n \\param argList an argument list for the formatter"]
    pub fn SoapySDR_vlogf(
        logLevel: SoapySDRLogLevel,
        format: *const ::std::os::raw::c_char,
        argList: va_list,
    );
}

// preserve any bindgen tests for all platforms out of an abundance of caution
// these get run as part of `cargo test`

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data_public {
    pub _locale_pctype: *const ::std::os::raw::c_ushort,
    pub _locale_mb_cur_max: ::std::os::raw::c_int,
    pub _locale_lc_codepage: ::std::os::raw::c_uint,
}

#[test]
fn bindgen_test_layout___crt_locale_data_public() {
    const UNINIT: ::std::mem::MaybeUninit<__crt_locale_data_public> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__crt_locale_data_public>(),
        16usize,
        concat!("Size of: ", stringify!(__crt_locale_data_public))
    );
    assert_eq!(
        ::std::mem::align_of::<__crt_locale_data_public>(),
        8usize,
        concat!("Alignment of ", stringify!(__crt_locale_data_public))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._locale_pctype) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_pctype)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._locale_mb_cur_max) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_mb_cur_max)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._locale_lc_codepage) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_lc_codepage)
        )
    );
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_pointers {
    pub locinfo: *mut __crt_locale_data,
    pub mbcinfo: *mut __crt_multibyte_data,
}

#[test]
fn bindgen_test_layout___crt_locale_pointers() {
    const UNINIT: ::std::mem::MaybeUninit<__crt_locale_pointers> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__crt_locale_pointers>(),
        16usize,
        concat!("Size of: ", stringify!(__crt_locale_pointers))
    );
    assert_eq!(
        ::std::mem::align_of::<__crt_locale_pointers>(),
        8usize,
        concat!("Alignment of ", stringify!(__crt_locale_pointers))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).locinfo) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_pointers),
            "::",
            stringify!(locinfo)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mbcinfo) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_pointers),
            "::",
            stringify!(mbcinfo)
        )
    );
}

pub type _locale_t = *mut __crt_locale_pointers;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Mbstatet {
    pub _Wchar: ::std::os::raw::c_ulong,
    pub _Byte: ::std::os::raw::c_ushort,
    pub _State: ::std::os::raw::c_ushort,
}

#[test]
fn bindgen_test_layout__Mbstatet() {
    const UNINIT: ::std::mem::MaybeUninit<_Mbstatet> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_Mbstatet>(),
        8usize,
        concat!("Size of: ", stringify!(_Mbstatet))
    );
    assert_eq!(
        ::std::mem::align_of::<_Mbstatet>(),
        4usize,
        concat!("Alignment of ", stringify!(_Mbstatet))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Wchar) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_Wchar)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Byte) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_Byte)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._State) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_State)
        )
    );
}

pub type mbstate_t = _Mbstatet;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_multibyte_data {
    pub _address: u8,
}
