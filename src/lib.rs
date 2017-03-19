//! [SoapySDR](https://github.com/pothosware/SoapySDR/wiki) provides a hardware abstraction layer
//! for transmitting and receiving with many software defined radio devices.
//!
//!

extern crate soapysdr_sys;
extern crate libc;
extern crate num_complex;

mod args;
pub use args::{Args, ArgsIterator};

mod arginfo;
pub use arginfo::ArgInfo;

mod device;
pub use device::{enumerate, Device, RxStream, TxStream, Error, ErrorCode, Direction, Range, Format, StreamSample};
