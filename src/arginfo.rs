use soapysdr_sys::*;
use std::slice;
use std::ptr;
use std::ffi::CStr;
use std::os::raw::c_char;


pub enum ArgType {

}

impl From<SoapySDRArgInfoType> for ArgType {
    fn from(_f: SoapySDRArgInfoType) -> ArgType {
        unimplemented!();
    }
}

pub struct ArgInfo {
    /// The key used to identify the argument
    pub key: String,

    /// The default value of the argument when not specified
    pub value: String,

    /// The displayable name of the argument
    pub name: Option<String>,

    ///  A brief description about the argument
    pub description: Option<String>,

    /// The units of the argument: dB, Hz, etc
    pub units: Option<String>,

    /// The data type of the argument
    pub data_type: ArgType,

    /// A discrete list of possible values.
    ///
    /// When specified, the argument should be restricted to this options set.
    pub options: Vec<(String, Option<String>)>,
}

unsafe fn required_string(s: *mut c_char) -> String {
    assert!(s != ptr::null_mut(), "Null string from SoapySDR");
    CStr::from_ptr(s).to_string_lossy().into()
}

unsafe fn optional_string(s: *mut c_char) -> Option<String> {
    if s != ptr::null_mut() {
        Some(CStr::from_ptr(s).to_string_lossy().into())
    } else {
        None
    }
}

pub unsafe fn arg_info_from_c(c: &SoapySDRArgInfo) -> ArgInfo {
    ArgInfo {
        key:         required_string(c.key),
        value:       required_string(c.value),
        name:        optional_string(c.name),
        description: optional_string(c.description),
        units:       optional_string(c.units),
        data_type:   ArgType::from(c.type_),
        options: {
            let option_vals = slice::from_raw_parts(c.options, c.numOptions);
            let option_names = slice::from_raw_parts(c.optionNames, c.numOptions);
            option_vals.iter().zip(option_names.iter()).map(|(&name, &val)| {
                (required_string(name), optional_string(val))
            }).collect()
        }
    }
}

pub unsafe fn arg_info_list_from_c(c: *mut SoapySDRArgInfo, len: usize) -> Vec<ArgInfo> {
    let r = slice::from_raw_parts(c, len).iter().map(|x| arg_info_from_c(x)).collect();
    SoapySDRArgInfoList_clear(c, len);
    r
}
