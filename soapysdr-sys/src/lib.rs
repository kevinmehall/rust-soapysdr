#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! This crate provides bindings for the [SoapySDR](https://github.com/pothosware/SoapySDR/wiki)
//! C API. See its [header file](https://github.com/pothosware/SoapySDR/blob/master/include/SoapySDR/Device.h)
//! for API documentation.

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
