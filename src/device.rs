use soapysdr_sys::*;
pub use soapysdr_sys::SoapySDRRange as Range;
use std::slice;
use std::ptr;
use std::sync::Arc;
use std::ffi::{ CStr, CString };
use std::os::raw::{c_int, c_char};
use std::os::raw::c_void;
use std::marker::PhantomData;

use super::{ Args, ArgInfo, StreamSample, Format };
use crate::arginfo::arg_info_from_c;

/// An error code from SoapySDR
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[non_exhaustive]
pub enum ErrorCode {
    /// Returned when read has a timeout.
    Timeout = -1,

    /// Returned for non-specific stream errors.
    StreamError = -2,

    /// Returned when read has data corruption.
    /// For example, the driver saw a malformed packet.
    Corruption = -3,

    /// Returned when read has an overflow condition.
    /// For example, and internal buffer has filled.
    Overflow = -4,

    /// Returned when a requested operation or flag setting
    /// is not supported by the underlying implementation.
    NotSupported = -5,

    /// Returned when a the device encountered a stream time
    /// which was expired (late) or too early to process.
    TimeError = -6,

    /// Returned when write caused an underflow condition.
    /// For example, a continuous stream was interrupted.
    Underflow = -7,

    /// Error without a specific code, see error string
    Other = 0,
}

impl ErrorCode {
    fn from_c(code: c_int) -> ErrorCode {
        match code {
            SOAPY_SDR_TIMEOUT       =>  ErrorCode::Timeout,
            SOAPY_SDR_STREAM_ERROR  =>  ErrorCode::StreamError,
            SOAPY_SDR_CORRUPTION    =>  ErrorCode::Corruption,
            SOAPY_SDR_OVERFLOW      =>  ErrorCode::Overflow,
            SOAPY_SDR_NOT_SUPPORTED =>  ErrorCode::NotSupported,
            SOAPY_SDR_TIME_ERROR    =>  ErrorCode::TimeError,
            SOAPY_SDR_UNDERFLOW     =>  ErrorCode::Underflow,
            _                       =>  ErrorCode::Other,
        }
    }
}

/// An error type combining an error code and a string message
#[derive(Clone, Debug, Hash)]
pub struct Error {
    pub code: ErrorCode,
    pub message: String,
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}: {}", self.code, self.message)
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        &self.message[..]
    }
}

/// Transmit or Receive
#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Direction {
    /// Transmit direction
    Tx = SOAPY_SDR_TX,

    /// Receive direction
    Rx = SOAPY_SDR_RX
}

impl From<Direction> for c_int {
    fn from(f: Direction) -> c_int {
        f as c_int
    }
}

struct DeviceInner {
    ptr: *mut SoapySDRDevice,
}

/// Device method implementations are required to be thread safe
unsafe impl Send for DeviceInner {}
unsafe impl Sync for DeviceInner {}

impl Drop for DeviceInner {
    fn drop(&mut self) {
        unsafe {
            SoapySDRDevice_unmake(self.ptr);
        }
    }
}

/// An opened SDR hardware device.
#[derive(Clone)]
pub struct Device {
    inner: Arc<DeviceInner>,
}

impl Device {
    fn from_ptr(ptr: *mut SoapySDRDevice) -> Device {
        Device {
            inner: Arc::new(DeviceInner {
                ptr,
            }),
        }
    }
}

fn last_error_str() -> String {
    unsafe {
        // Capture error string from thread local storage
        CStr::from_ptr(SoapySDRDevice_lastError()).to_string_lossy().into()
    }
}

fn check_error<T>(r: T) -> Result<T, Error> {
    unsafe {
        if SoapySDRDevice_lastStatus() == 0 {
            Ok(r)
        } else {
            Err(Error {
                code: ErrorCode::Other,
                message: last_error_str(),
            })
        }
    }
}

fn check_ret_error(r: c_int) -> Result<(), Error> {
    if r == 0 {
        Ok(())
    } else {
        Err(Error {
            code: ErrorCode::from_c(r),
            message: last_error_str(),
        })
    }
}

fn len_result(ret: c_int) -> Result<c_int, Error> {
    if ret >= 0 {
        Ok(ret)
    } else {
        Err(Error {
            code: ErrorCode::from_c(ret),
            message: last_error_str(),
        })
    }
}

unsafe fn string_result(r: *mut c_char) -> Result<String, Error> {
    let ptr: *mut c_char = check_error(r)?;
    let ret = CStr::from_ptr(ptr).to_string_lossy().into();
    SoapySDR_free(ptr as *mut c_void);
    Ok(ret)
}

unsafe fn string_list_result<F: FnOnce(*mut usize) -> *mut *mut c_char>(f: F) -> Result<Vec<String>, Error> {
    let mut len: usize = 0;
    let mut ptr = check_error(f(&mut len as *mut _))?;
    let ret = slice::from_raw_parts(ptr, len).iter()
        .map(|&p| CStr::from_ptr(p).to_string_lossy().into())
        .collect();
    SoapySDRStrings_clear(&mut ptr as *mut _, len);
    Ok(ret)
}

unsafe fn arg_info_result<F: FnOnce(*mut usize) -> *mut SoapySDRArgInfo>(f: F) -> Result<Vec<ArgInfo>, Error> {
    let mut len: usize = 0;
    let ptr = check_error(f(&mut len as *mut _))?;
    let r = slice::from_raw_parts(ptr, len).iter().map(|x| arg_info_from_c(x)).collect();
    SoapySDRArgInfoList_clear(ptr, len);
    Ok(r)
}

unsafe fn list_result<T: Copy, F: FnOnce(*mut usize) -> *mut T>(f: F) -> Result<Vec<T>, Error> {
    let mut len: usize = 0;
    let ptr = check_error(f(&mut len as *mut _))?;
    let ret = slice::from_raw_parts(ptr, len).to_owned();
    SoapySDR_free(ptr as *mut c_void);
    Ok(ret)
}

fn optional_string_arg<S: AsRef<str>>(optstr: Option<S>) -> CString {
    match optstr {
        Some(s) => CString::new(s.as_ref()).expect("Optional arg string contains null"),
        None => CString::new("").unwrap(),
    }
}

/// Enumerate a list of available devices on the system.
///
/// `args`: a set of arguments to filter the devices returned.
///
/// # Example (list all devices)
/// ```
/// for dev in soapysdr::enumerate("").unwrap() {
///     println!("{}", dev);
/// }
/// ```
///
/// This function returns a list of argument lists that can be passed to `Device::new()` to
/// open the device.
pub fn enumerate<A: Into<Args>>(args: A) -> Result<Vec<Args>, Error> {
    unsafe {
        let mut len: usize = 0;
        let devs = check_error(SoapySDRDevice_enumerate(args.into().as_raw_const(), &mut len as *mut _))?;
        let args = slice::from_raw_parts(devs, len).iter().map(|&arg| Args::from_raw(arg)).collect();
        SoapySDR_free(devs as *mut c_void);
        Ok(args)
    }
}

impl Device {
    /// Find and open a device matching a set of filters.
    ///
    /// # Example
    /// ```
    /// let mut d = soapysdr::Device::new("type=null").unwrap();
    /// ```
    pub fn new<A: Into<Args>>(args: A) -> Result<Device, Error> {
        unsafe {
            let d = check_error(SoapySDRDevice_make(args.into().as_raw_const()))?;
            Ok(Device::from_ptr(d))
        }
    }

    #[doc(hidden)]
    pub fn null_device() -> Device {
        Device::new("type=null").unwrap()
    }

    /// A key that uniquely identifies the device driver.
    ///
    /// This key identifies the underlying implementation.
    /// Several variants of a product may share a driver.
    pub fn driver_key(&self) -> Result<String, Error> {
        unsafe {
            string_result(SoapySDRDevice_getDriverKey(self.inner.ptr))
        }
    }

    /// A key that uniquely identifies the hardware.
    ///
    /// This key should be meaningful to the user to optimize for the underlying hardware.
    pub fn hardware_key(&self) -> Result<String, Error> {
        unsafe {
            string_result(SoapySDRDevice_getDriverKey(self.inner.ptr))
        }
    }

    /// Query a dictionary of available device information.
    ///
    /// This dictionary can any number of values like
    /// vendor name, product name, revisions, serials...
    ///
    /// This information can be displayed to the user
    /// to help identify the instantiated device.
    pub fn hardware_info(&self) -> Result<Args, Error> {
        unsafe {
            check_error(SoapySDRDevice_getHardwareInfo(self.inner.ptr)).map(|x| Args::from_raw(x))
        }
    }

    /// Get the mapping configuration string.
    pub fn frontend_mapping(&self, direction: Direction) -> Result<String, Error> {
        unsafe {
            string_result(SoapySDRDevice_getFrontendMapping(self.inner.ptr, direction.into()))
        }
    }

    /// Set the frontend mapping of available DSP units to RF frontends.
    ///
    /// This controls channel mapping and channel availability.
    pub fn set_frontend_mapping<S: Into<Vec<u8>>>(&self, direction: Direction, mapping: S) -> Result<(), Error> {
        unsafe {
            let mapping_c = CString::new(mapping).expect("Mapping contains null byte");
            SoapySDRDevice_setFrontendMapping(self.inner.ptr, direction.into(), mapping_c.as_ptr());
            check_error(())
        }
    }

    /// Get a number of channels given the streaming direction
    pub fn num_channels(&self, direction: Direction) -> Result<usize, Error> {
        unsafe {
            check_error(SoapySDRDevice_getNumChannels(self.inner.ptr, direction.into()))
        }
    }

    /// Get channel info given the streaming direction
    pub fn channel_info(&self, direction: Direction, channel: usize) -> Result<Args, Error> {
        unsafe {
            check_error(SoapySDRDevice_getChannelInfo(self.inner.ptr, direction.into(), channel)).map(|x| Args::from_raw(x))
        }
    }

    /// Find out if the specified channel is full or half duplex.
    ///
    /// Returns `true` for full duplex, `false` for half duplex.
    pub fn full_duplex(&self, direction: Direction, channel: usize) -> Result<bool, Error> {
        unsafe {
            check_error(SoapySDRDevice_getFullDuplex(self.inner.ptr, direction.into(), channel))
        }
    }

    /// Query a list of the available stream formats.
    pub fn stream_formats(&self, direction: Direction, channel: usize) -> Result<Vec<Format>, Error> {
        unsafe {
            let mut len: usize = 0;
            let mut ptr = check_error(SoapySDRDevice_getStreamFormats(self.inner.ptr, direction.into(), channel, &mut len as *mut _))?;
            let ret = slice::from_raw_parts(ptr, len).iter()
                .flat_map(|&p| CStr::from_ptr(p).to_str().ok())
                .flat_map(|s| s.parse().ok())
                .collect();
            SoapySDRStrings_clear(&mut ptr as *mut _, len);
            Ok(ret)
        }
    }

    /// Get the hardware's native stream format and full-scale value for this channel.
    ///
    /// This is the format used by the underlying transport layer,
    /// and the direct buffer access API calls (when available).
    pub fn native_stream_format(&self, direction: Direction, channel: usize) -> Result<(Format, f64), Error> {
        unsafe {
            let mut fullscale: f64 = 0.0;
            let ptr = check_error(SoapySDRDevice_getNativeStreamFormat(self.inner.ptr, direction.into(), channel, &mut fullscale as *mut _))?;

            let format = CStr::from_ptr(ptr).to_str().ok()
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| Error { code: ErrorCode::Other, message: "Invalid stream format returned by SoapySDR".into()})?;

            Ok((format, fullscale))
        }
    }

    /// Query the argument info description for stream args.
    pub fn stream_args_info(&self, direction: Direction, channel: usize) -> Result<Vec<ArgInfo>, Error> {
        unsafe {
            arg_info_result(|len_ptr| SoapySDRDevice_getStreamArgsInfo(self.inner.ptr, direction.into(), channel, len_ptr))
        }
    }

    ///  Initialize an RX stream given a list of channels
    pub fn rx_stream<E: StreamSample>(&self, channels: &[usize]) -> Result<RxStream<E>, Error> {
        self.rx_stream_args(channels, ())
    }

    ///  Initialize an RX stream given a list of channels and stream arguments.
    pub fn rx_stream_args<E: StreamSample, A: Into<Args>>(&self, channels: &[usize], args: A) -> Result<RxStream<E>, Error> {
        unsafe {
            let mut stream: *mut SoapySDRStream = ptr::null_mut();
            check_error(SoapySDRDevice_setupStream(self.inner.ptr,
                &mut stream as *mut _,
                Direction::Rx.into(),
                E::STREAM_FORMAT.as_ptr(),
                channels.as_ptr(), channels.len(),
                args.into().as_raw_const()
            )).map(|_| RxStream {
                device: self.clone(),
                handle: stream,
                nchannels: channels.len(),
                flags: 0,
                time_ns: 0,
                active: false,
                phantom: PhantomData,
            })
        }
    }

    /// Initialize a TX stream given a list of channels and stream arguments.
    pub fn tx_stream<E: StreamSample>(&self, channels: &[usize]) -> Result<TxStream<E>, Error> {
        self.tx_stream_args(channels, ())
    }

    /// Initialize a TX stream given a list of channels and stream arguments.
    pub fn tx_stream_args<E: StreamSample, A: Into<Args>>(&self, channels: &[usize], args: A) -> Result<TxStream<E>, Error> {
        unsafe {
            let mut stream: *mut SoapySDRStream = ptr::null_mut();
            check_error(SoapySDRDevice_setupStream(self.inner.ptr,
                &mut stream as *mut _,
                Direction::Tx.into(),
                E::STREAM_FORMAT.as_ptr(),
                channels.as_ptr(), channels.len(),
                args.into().as_raw_const()
            )).map(|_| TxStream {
                device: self.clone(),
                handle: stream,
                nchannels: channels.len(),
                active: false,
                phantom: PhantomData,
            })
        }
    }

    /// Get a list of available antennas to select on a given chain.
    pub fn antennas(&self, direction: Direction, channel: usize) -> Result<Vec<String>, Error> {
        unsafe {
            string_list_result(|len_ptr| SoapySDRDevice_listAntennas(self.inner.ptr, direction.into(), channel, len_ptr))
        }
    }

    /// Set the selected antenna on a chain.
    pub fn set_antenna<S: Into<Vec<u8>>>(&self, direction: Direction, channel: usize, name: S) -> Result<(), Error> {
        unsafe {
            let name_c = CString::new(name).expect("Antenna name contains null byte");
            SoapySDRDevice_setAntenna(self.inner.ptr, direction.into(), channel, name_c.as_ptr());
            check_error(())
        }
    }

    /// Get the selected antenna on a chain.
    pub fn antenna(&self, direction: Direction, channel: usize) -> Result<String, Error> {
        unsafe {
            string_result(SoapySDRDevice_getAntenna(self.inner.ptr, direction.into(), channel))
        }
    }

    /// Does the device support automatic DC offset corrections?
    ///
    /// Returns true if automatic corrections are supported
    pub fn has_dc_offset_mode(&self, direction: Direction, channel: usize) -> Result<bool, Error> {
        unsafe {
            check_error(SoapySDRDevice_hasDCOffsetMode(self.inner.ptr, direction.into(), channel))
        }
    }

    /// Enable or disable automatic DC offset corrections mode.
    pub fn set_dc_offset_mode(&self, direction: Direction, channel: usize, automatic: bool) -> Result<(), Error> {
        unsafe {
            SoapySDRDevice_setDCOffsetMode(self.inner.ptr, direction.into(), channel, automatic);
            check_error(())
        }
    }

    /// Returns true if automatic DC offset mode is enabled
    pub fn dc_offset_mode(&self, direction: Direction, channel: usize) -> Result<bool, Error> {
        unsafe {
            check_error(SoapySDRDevice_getDCOffsetMode(self.inner.ptr, direction.into(), channel))
        }
    }

    /// Does the device support frontend DC offset corrections?
    ///
    /// Returns true if manual corrections are supported
    pub fn has_dc_offset(&self, direction: Direction, channel: usize) -> Result<bool, Error> {
        unsafe {
            check_error(SoapySDRDevice_hasDCOffset(self.inner.ptr, direction.into(), channel))
        }
    }

    /// Set the frontend DC offset correction.
    ///
    /// The offsets are configured for each of the I and Q components (1.0 max)
    pub fn set_dc_offset(&self, direction: Direction, channel: usize, offset_i: f64, offset_q: f64) -> Result<(), Error> {
        unsafe {
            SoapySDRDevice_setDCOffset(self.inner.ptr, direction.into(), channel, offset_i, offset_q);
            check_error(())
        }
    }

    /// Get the frontend DC offset correction for (I, Q), 1.0 max
    pub fn dc_offset(&self, direction: Direction, channel: usize) -> Result<(f64, f64), Error> {
        unsafe {
            let mut i: f64 = 0.0;
            let mut q: f64 = 0.0;
            SoapySDRDevice_getDCOffset(self.inner.ptr, direction.into(), channel, &mut i as *mut _, &mut q as *mut _);
            check_error((i, q))
        }
    }

    /// Does the device support frontend IQ balance correction?
    ///
    /// Returns true if IQ balance corrections are supported.
    pub fn has_iq_balance(&self, direction: Direction, channel: usize) -> Result<bool, Error> {
        unsafe {
            check_error(SoapySDRDevice_hasIQBalance(self.inner.ptr, direction.into(), channel))
        }
    }

    /// Set the frontend IQ balance correction
    ///
    /// The correction is configured for each of the I and Q components (1.0 max)
    pub fn set_iq_balance(&self, direction: Direction, channel: usize, balance_i: f64, balance_q: f64) -> Result<(), Error> {
        unsafe {
            SoapySDRDevice_setIQBalance(self.inner.ptr, direction.into(), channel, balance_i, balance_q);
            check_error(())
        }
    }

    /// Get the frontend IQ balance correction for (I, Q), 1.0 max
    pub fn iq_balance(&self, direction: Direction, channel: usize) -> Result<(f64, f64), Error> {
        unsafe {
            let mut i: f64 = 0.0;
            let mut q: f64 = 0.0;
            SoapySDRDevice_getIQBalance(self.inner.ptr, direction.into(), channel, &mut i as *mut _, &mut q as *mut _);
            check_error((i, q))
        }
    }

    /// List available amplification elements.
    ///
    /// Elements should be in order RF to baseband.
    pub fn list_gains(&self, direction: Direction, channel: usize) -> Result<Vec<String>, Error> {
        unsafe {
            string_list_result(|len_ptr| SoapySDRDevice_listGains(self.inner.ptr, direction.into(), channel, len_ptr))
        }
    }

    /// Does the device support automatic gain control?
    pub fn has_gain_mode(&self, direction: Direction, channel: usize) -> Result<bool, Error> {
        unsafe {
            check_error(SoapySDRDevice_hasGainMode(self.inner.ptr, direction.into(), channel))
        }
    }

    /// Enable or disable automatic gain control.
    pub fn set_gain_mode(&self, direction: Direction, channel: usize, automatic: bool) -> Result<(), Error> {
        unsafe {
            SoapySDRDevice_setGainMode(self.inner.ptr, direction.into(), channel, automatic);
            check_error(())
        }
    }

    /// Returns true if automatic gain control is enabled
    pub fn gain_mode(&self, direction: Direction, channel: usize) -> Result<bool, Error> {
        unsafe {
            check_error(SoapySDRDevice_getGainMode(self.inner.ptr, direction.into(), channel))
        }
    }

    /// Set the overall amplification in a chain.
    ///
    /// The gain will be distributed automatically across available elements.
    ///
    /// `gain`: the new amplification value in dB
    pub fn set_gain(&self, direction: Direction, channel: usize, gain: f64) -> Result<(), Error> {
        unsafe {
            SoapySDRDevice_setGain(self.inner.ptr, direction.into(), channel, gain);
            check_error(())
        }
    }

    /// Get the overall value of the gain elements in a chain in dB.
    pub fn gain(&self, direction: Direction, channel: usize) -> Result<f64, Error> {
        unsafe {
            check_error(SoapySDRDevice_getGain(self.inner.ptr, direction.into(), channel))
        }
    }

    /// Get the overall range of possible gain values.
    pub fn gain_range(&self, direction: Direction, channel: usize) -> Result<Range, Error> {
        unsafe {
            check_error(SoapySDRDevice_getGainRange(self.inner.ptr, direction.into(), channel))
        }
    }

    /// Set the value of a amplification element in a chain.
    ///
    /// # Arguments
    /// * `name`: the name of an amplification element from `Device::list_gains`
    /// * `gain`: the new amplification value in dB
    pub fn set_gain_element<S: Into<Vec<u8>>>(&self, direction: Direction, channel: usize, name: S, gain: f64) -> Result<(), Error> {
        unsafe {
            let name_c = CString::new(name).expect("Gain name contains null byte");
            SoapySDRDevice_setGainElement(self.inner.ptr, direction.into(), channel, name_c.as_ptr(), gain);
            check_error(())
        }
    }

    /// Get the value of an individual amplification element in a chain in dB.
    pub fn gain_element<S: Into<Vec<u8>>>(&self, direction: Direction, channel: usize, name: S) -> Result<f64, Error> {
        unsafe {
            let name_c = CString::new(name).expect("Gain name contains null byte");
            check_error(SoapySDRDevice_getGainElement(self.inner.ptr, direction.into(), channel, name_c.as_ptr()))
        }
    }

    /// Get the range of possible gain values for a specific element.
    pub fn gain_element_range<S: Into<Vec<u8>>>(&self, direction: Direction, channel: usize, name: S) -> Result<Range, Error> {
        unsafe {
            let name_c = CString::new(name).expect("Gain name contains null byte");
            check_error(SoapySDRDevice_getGainElementRange(self.inner.ptr, direction.into(), channel, name_c.as_ptr()))
        }
    }

    /// Get the ranges of overall frequency values.
    pub fn frequency_range(&self, direction: Direction, channel: usize) -> Result<Vec<Range>, Error> {
        unsafe {
            list_result(|len_ptr| SoapySDRDevice_getFrequencyRange(self.inner.ptr, direction.into(), channel, len_ptr))
        }
    }

    /// Get the overall center frequency of the chain.
    ///
    ///   - For RX, this specifies the down-conversion frequency.
    ///   - For TX, this specifies the up-conversion frequency.
    ///
    /// Returns the center frequency in Hz.
    pub fn frequency(&self, direction: Direction, channel: usize) -> Result<f64, Error> {
        unsafe {
            check_error(SoapySDRDevice_getFrequency(self.inner.ptr, direction.into(), channel))
        }
    }

    /// Set the center frequency of the chain.
    ///
    ///   - For RX, this specifies the down-conversion frequency.
    ///   - For TX, this specifies the up-conversion frequency.
    ///
    /// The default implementation of `set_frequency` will tune the "RF"
    /// component as close as possible to the requested center frequency in Hz.
    /// Tuning inaccuracies will be compensated for with the "BB" component.
    ///
    /// The `args` can be used to augment the tuning algorithm.
    ///
    ///   - Use `"OFFSET"` to specify an "RF" tuning offset,
    ///     usually with the intention of moving the LO out of the passband.
    ///     The offset will be compensated for using the "BB" component.
    ///   - Use the name of a component for the key and a frequency in Hz
    ///     as the value (any format) to enforce a specific frequency.
    ///     The other components will be tuned with compensation
    ///     to achieve the specified overall frequency.
    ///   - Use the name of a component for the key and the value `"IGNORE"`
    ///     so that the tuning algorithm will avoid altering the component.
    ///   - Vendor specific implementations can also use the same args to augment
    ///     tuning in other ways such as specifying fractional vs integer N tuning.
    ///
    pub fn set_frequency<A: Into<Args>>(&self, direction: Direction, channel: usize, frequency: f64, args: A) -> Result<(), Error> {
        unsafe {
            SoapySDRDevice_setFrequency(self.inner.ptr, direction.into(), channel, frequency, args.into().as_raw_const());
            check_error(())
        }
    }

    /// List available tunable elements in the chain.
    ///
    /// Elements should be in order RF to baseband.
    pub fn list_frequencies(&self, direction: Direction, channel: usize) -> Result<Vec<String>, Error> {
        unsafe {
            string_list_result(|len_ptr| SoapySDRDevice_listFrequencies(self.inner.ptr, direction.into(), channel, len_ptr))
        }
    }

    /// Get the range of tunable values for the specified element.
    pub fn component_frequency_range<S: Into<Vec<u8>>>(&self, direction: Direction, channel: usize, name: S) -> Result<Vec<Range>, Error> {
        unsafe {
            let name_c = CString::new(name).expect("Component name contains null byte");
            list_result(|len_ptr| SoapySDRDevice_getFrequencyRangeComponent(self.inner.ptr, direction.into(), channel, name_c.as_ptr(), len_ptr))
        }
    }

    /// Get the frequency of a tunable element in the chain.
    pub fn component_frequency<S: Into<Vec<u8>>>(&self, direction: Direction, channel: usize, name: S) -> Result<f64, Error> {
        unsafe {
            let name_c = CString::new(name).expect("Component name contains null byte");
            check_error(SoapySDRDevice_getFrequencyComponent(self.inner.ptr, direction.into(), channel, name_c.as_ptr()))
        }
    }

    /// Tune the center frequency of the specified element.
    ///
    ///   - For RX, this specifies the down-conversion frequency.
    ///   - For TX, this specifies the up-conversion frequency.
    ///
    /// Recommended names used to represent tunable components:
    ///
    ///   - "CORR" - freq error correction in PPM
    ///   - "RF" - frequency of the RF frontend
    ///   - "BB" - frequency of the baseband DSP
    ///
    pub fn set_component_frequency<S: Into<Vec<u8>>, A: Into<Args>>(&self, direction: Direction, channel: usize, name: S, frequency: f64, args: A) -> Result<(), Error> {
        unsafe {
            let name_c = CString::new(name).expect("Component name contains null byte");
            SoapySDRDevice_setFrequencyComponent(self.inner.ptr, direction.into(), channel, name_c.as_ptr(), frequency, args.into().as_raw_const());
            check_error(())
        }
    }

    /// Query the argument info description for tune args.
    pub fn frequency_args_info(&self, direction: Direction, channel: usize) -> Result<Vec<ArgInfo>, Error> {
        unsafe {
            arg_info_result(|len_ptr| SoapySDRDevice_getFrequencyArgsInfo(self.inner.ptr, direction.into(), channel, len_ptr))
        }
    }

    /// Get the baseband sample rate of the chain in samples per second.
    pub fn sample_rate(&self, direction: Direction, channel: usize) -> Result<f64, Error> {
        unsafe {
            check_error(SoapySDRDevice_getSampleRate(self.inner.ptr, direction.into(), channel))
        }
    }

    /// Set the baseband sample rate of the chain in samples per second.
    pub fn set_sample_rate(&self, direction: Direction, channel: usize, rate: f64) -> Result<(), Error> {
        unsafe {
            SoapySDRDevice_setSampleRate(self.inner.ptr, direction.into(), channel, rate);
            check_error(())
        }
    }

    /// Get the range of possible baseband sample rates.
    pub fn get_sample_rate_range(&self, direction: Direction, channel: usize) -> Result<Vec<Range>, Error> {
        unsafe {
            list_result(|len_ptr| SoapySDRDevice_getSampleRateRange(self.inner.ptr, direction.into(), channel, len_ptr))
        }
    }

    /// Get the baseband filter width of the chain in Hz
    pub fn bandwidth(&self, direction: Direction, channel: usize) -> Result<f64, Error> {
        unsafe {
            check_error(SoapySDRDevice_getBandwidth(self.inner.ptr, direction.into(), channel))
        }
    }

    /// Set the baseband filter width of the chain in Hz
    pub fn set_bandwidth(&self, direction: Direction, channel: usize, bandwidth: f64) -> Result<(), Error> {
        unsafe {
            SoapySDRDevice_setBandwidth(self.inner.ptr, direction.into(), channel, bandwidth);
            check_error(())
        }
    }

    /// Get the ranges of possible baseband filter widths.
    pub fn bandwidth_range(&self, direction: Direction, channel: usize) -> Result<Vec<Range>, Error> {
        unsafe {
            list_result(|len_ptr| SoapySDRDevice_getBandwidthRange(self.inner.ptr, direction.into(), channel, len_ptr))
        }
    }

    /// List time sources
    pub fn list_time_sources(&self) -> Result<Vec<String>, Error> {
        unsafe { string_list_result(|len_ptr| SoapySDRDevice_listTimeSources(self.inner.ptr, len_ptr)) }
    }

    /// Get the current time source
    pub fn get_time_source(&self) -> Result<String, Error> {
        unsafe { string_result(SoapySDRDevice_getTimeSource(self.inner.ptr)) }
    }

    /// Set the current time source
    pub fn set_time_source<S: Into<Vec<u8>>>(&self, time_source: S) -> Result<(), Error> {
        let time_source = CString::new(time_source).expect("Time source contained null");
        unsafe {
            SoapySDRDevice_setTimeSource(self.inner.ptr, time_source.as_ptr());
            check_error(())
        }
    }

    /// Check whether there is a given hardware time source.
    /// Hardware time sources are not the same as time sources (at least for UHD Devices)
    /// UHD supported hw time sources: "PPS" or "" (i.e. None)
    pub fn has_hardware_time(
        &self,
        hw_time_source: Option<&str>,
    ) -> Result<bool, Error> {
        let hw_time_source = optional_string_arg(hw_time_source);
        unsafe {
            let has_hw_time = SoapySDRDevice_hasHardwareTime(self.inner.ptr, hw_time_source.as_ptr());
            check_error(has_hw_time)
        }
    }

    /// Get the current timestamp in ns
    pub fn get_hardware_time(
        &self,
        hw_time_source: Option<&str>,
    ) -> Result<i64, Error> {
        let hw_time_source = optional_string_arg(hw_time_source);
        unsafe {
            let tstamp = SoapySDRDevice_getHardwareTime(self.inner.ptr, hw_time_source.as_ptr());
            check_error(tstamp)
        }
    }

    /// Set the current hardware timestmap for the given source
    /// UHD supported hardware times: "CMD","PPS","UNKNOWN_PPS"
    pub fn set_hardware_time(
        &self,
        hw_time_source: Option<&str>,
        new_time_ns: i64,
    ) -> Result<(), Error> {
        let hw_time_source = optional_string_arg(hw_time_source);
        unsafe {
            SoapySDRDevice_setHardwareTime(self.inner.ptr, new_time_ns, hw_time_source.as_ptr());
            check_error(())
        }
    }

    /// List clock sources
    pub fn list_clock_sources(&self) -> Result<Vec<String>, Error> {
        unsafe { string_list_result(|len_ptr| SoapySDRDevice_listClockSources(self.inner.ptr, len_ptr)) }
    }

    /// Get the current clock source
    pub fn get_clock_source(&self) -> Result<String, Error> {
        unsafe { string_result(SoapySDRDevice_getClockSource(self.inner.ptr)) }
    }

    /// Set the current clock source
    pub fn set_clock_source<S: Into<Vec<u8>>>(&self, clock_source: S) -> Result<(), Error> {
        let clock_source = CString::new(clock_source).expect("clock source contained null");
        unsafe {
            SoapySDRDevice_setClockSource(self.inner.ptr, clock_source.as_ptr());
            check_error(())
        }
    }

    // TODO: sensors

    // TODO: registers

    /// Write a setting
    pub fn write_setting<S: Into<Vec<u8>>>(&self, key: S, value: S) -> Result<(), Error> {
        let key = CString::new(key).expect("key must not contain null byte");
        let value = CString::new(value).expect("value must not contain null byte");
        unsafe {
            check_ret_error(SoapySDRDevice_writeSetting(self.inner.ptr, key.as_ptr(), value.as_ptr()))?;
            Ok(())
        }
    }

    /// Read a setting
    pub fn read_setting<S: Into<Vec<u8>>>(&self, key: S) -> Result<String, Error> {
        let key = CString::new(key).expect("key must not contain null byte");
        unsafe {
            string_result(SoapySDRDevice_readSetting(self.inner.ptr, key.as_ptr()))
        }
    }

    // TODO: gpio

    // TODO: I2C

    // TODO: SPI

    // TODO: UART

}

/// A stream open for receiving.
///
/// To obtain a RxStream, call [Device::rx_stream]. The type parameter `E` represents the type
/// of this stream's samples.
///
/// Streams may involve multiple channels.
pub struct RxStream<E: StreamSample> {
    device: Device,
    handle: *mut SoapySDRStream,
    nchannels: usize,
    flags: i32,
    time_ns: i64,
    active: bool,
    phantom: PhantomData<fn(&mut[E])>,
}

/// Streams may only be used on one thread at a time but may be sent between threads
unsafe impl<E: StreamSample> Send for RxStream<E> {}

impl<E: StreamSample> Drop for RxStream<E> {
    fn drop(&mut self) {
        unsafe {
            if self.active {
                self.deactivate(None).ok();
            }
            SoapySDRDevice_closeStream(self.device.inner.ptr, self.handle);
        }
    }
}

impl<E: StreamSample> RxStream<E> {
    /// Get the stream's maximum transmission unit (MTU) in number of elements.
    ///
    /// The MTU specifies the maximum payload transfer in a stream operation.
    /// This value can be used as a stream buffer allocation size that can
    /// best optimize throughput given the underlying stream implementation.
    pub fn mtu(&self) -> Result<usize, Error> {
        unsafe {
            check_error(SoapySDRDevice_getStreamMTU(self.device.inner.ptr, self.handle))
        }
    }

    /// Activate a stream.
    ///
    /// Call `activate` to enable a stream before using `read()`
    ///
    /// # Arguments:
    ///   * `time_ns` -- optional activation time in nanoseconds
    pub fn activate(&mut self, time_ns: Option<i64>) -> Result<(), Error> {
        if self.active { return Err(Error { code: ErrorCode::Other, message: "Stream is already active".into() }); }
        unsafe {
            let flags = if time_ns.is_some() { SOAPY_SDR_HAS_TIME as i32 } else { 0 };
            check_ret_error(SoapySDRDevice_activateStream(self.device.inner.ptr, self.handle, flags, time_ns.unwrap_or(0), 0))?;
            self.active = true;
            Ok(())
        }
    }

    // TODO: activate_burst()

    /// Deactivate a stream.
    /// The implementation will control switches or halt data flow.
    ///
    /// # Arguments:
    ///   * `time_ns` -- optional deactivation time in nanoseconds
    pub fn deactivate(&mut self, time_ns: Option<i64>) -> Result<(), Error> {
        if !self.active { return Err(Error { code: ErrorCode::Other, message: "Stream is not active".into() }); }
        unsafe {
            let flags = if time_ns.is_some() { SOAPY_SDR_HAS_TIME as i32 } else { 0 };
            check_ret_error(SoapySDRDevice_deactivateStream(self.device.inner.ptr, self.handle, flags, time_ns.unwrap_or(0)))?;
            self.active = false;
            Ok(())
        }
    }

    /// Read samples from the stream into the provided buffers.
    ///
    /// `buffers` contains one destination slice for each channel of this stream.
    ///
    /// Returns the number of samples read, which may be smaller than the size of the passed arrays.
    ///
    /// # Panics
    ///  * If `buffers` is not the same length as the `channels` array passed to `Device::rx_stream`.
    pub fn read(&mut self, buffers: &mut [&mut [E]], timeout_us: i64) -> Result<usize, Error> {
        unsafe {
            assert!(buffers.len() == self.nchannels);

            let num_samples = buffers.iter().map(|b| b.len()).min().unwrap_or(0);

            //TODO: avoid this allocation
            let buf_ptrs = buffers.iter().map(|b| b.as_ptr()).collect::<Vec<_>>();

            self.flags = 0;
            let len = len_result(SoapySDRDevice_readStream(
                self.device.inner.ptr,
                self.handle,
                buf_ptrs.as_ptr() as *const *mut _,
                num_samples,
                &mut self.flags as *mut _,
                &mut self.time_ns as *mut _,
                timeout_us as _
            ))?;

            Ok(len as usize)
        }
    }

}

/// A stream open for transmitting.
///
/// To obtain a TxStream, call [Device::tx_stream]. The type parameter `E` represents the type
/// of this stream's samples.
///
/// Streams may involve multiple channels.
pub struct TxStream<E: StreamSample> {
    device: Device,
    handle: *mut SoapySDRStream,
    nchannels: usize,
    active: bool,
    phantom: PhantomData<fn(&[E])>,
}

/// Streams may only be used on one thread at a time but may be sent between threads
unsafe impl<E: StreamSample> Send for TxStream<E> {}

impl<E: StreamSample> Drop for TxStream<E> {
    fn drop(&mut self) {
        unsafe {
            if self.active {
                self.deactivate(None).ok();
            }
            SoapySDRDevice_closeStream(self.device.inner.ptr, self.handle);
        }
    }
}

impl<E: StreamSample> TxStream<E> {
    /// Get the stream's maximum transmission unit (MTU) in number of elements.
    ///
    /// The MTU specifies the maximum payload transfer in a stream operation.
    /// This value can be used as a stream buffer allocation size that can
    /// best optimize throughput given the underlying stream implementation.
    pub fn mtu(&self) -> Result<usize, Error> {
        unsafe {
            check_error(SoapySDRDevice_getStreamMTU(self.device.inner.ptr, self.handle))
        }
    }

    /// Activate a stream.
    ///
    /// Call `activate` to enable a stream before using `write()`
    ///
    /// # Arguments:
    ///   * `time_ns` -- optional activation time in nanoseconds
    pub fn activate(&mut self, time_ns: Option<i64>) -> Result<(), Error> {
        if self.active { return Err(Error { code: ErrorCode::Other, message: "Stream is already active".into() }); }
        unsafe {
            let flags = if time_ns.is_some() { SOAPY_SDR_HAS_TIME as i32 } else { 0 };
            check_ret_error(SoapySDRDevice_activateStream(self.device.inner.ptr, self.handle, flags, time_ns.unwrap_or(0), 0))?;
            self.active = true;
            Ok(())
        }
    }

    /// Deactivate a stream.
    /// The implementation will control switches or halt data flow.
    ///
    /// # Arguments:
    ///   * `time_ns` -- optional deactivation time in nanoseconds
    pub fn deactivate(&mut self, time_ns: Option<i64>) -> Result<(), Error> {
        if !self.active { return Err(Error { code: ErrorCode::Other, message: "Stream is not active".into() }); }
        unsafe {
            let flags = if time_ns.is_some() { SOAPY_SDR_HAS_TIME as i32 } else { 0 };
            check_ret_error(SoapySDRDevice_deactivateStream(self.device.inner.ptr, self.handle, flags, time_ns.unwrap_or(0)))?;
            self.active = false;
            Ok(())
        }
    }

    /// Attempt to write samples to the device from the provided buffer.
    ///
    /// The stream must first be [activated](TxStream::activate).
    ///
    /// `buffers` contains one source slice for each channel of the stream.
    ///
    /// `at_ns` is an optional nanosecond precision device timestamp at which
    /// the device is to begin the transmission (c.f. [get_hardware_time](Device::get_hardware_time)).
    ///
    /// `end_burst` indicates when this packet ends a burst transmission.
    ///
    /// Returns the number of samples written, which may be smaller than the size of the passed arrays.
    ///
    /// # Panics
    ///  * If `buffers` is not the same length as the `channels` array passed to `Device::tx_stream`.
    ///  * If all the buffers in `buffers` are not the same length.
    pub fn write(&mut self, buffers: &[&[E]], at_ns: Option<i64>, end_burst: bool, timeout_us: i64) -> Result<usize, Error> {
        unsafe {
            assert!(buffers.len() == self.nchannels, "Number of buffers must equal number of channels on stream");

            let mut buf_ptrs = Vec::with_capacity(self.nchannels);
            let num_elems = buffers.get(0).map_or(0, |x| x.len());
            for buf in buffers {
                assert_eq!(buf.len(), num_elems, "All buffers must be the same length");
                buf_ptrs.push(buf.as_ptr());
            }

            let mut flags = 0;

            if at_ns.is_some() {
                flags |= SOAPY_SDR_HAS_TIME as i32;
            }

            if end_burst {
                flags |= SOAPY_SDR_END_BURST as i32;
            }

            let len = len_result(SoapySDRDevice_writeStream(
                self.device.inner.ptr,
                self.handle,
                buf_ptrs.as_ptr() as *const *const _,
                num_elems,
                &mut flags as *mut _,
                at_ns.unwrap_or(0),
                timeout_us as _
            ))?;

            Ok(len as usize)
        }
    }

    /// Write all samples to the device.
    ///
    /// This method repeatedly calls [write](TxStream::write) until the entire provided buffer has
    /// been written.
    ///
    /// The stream must first be [activated](TxStream::activate).
    ///
    /// `buffers` contains one source slice for each channel of the stream.
    ///
    /// `at_ns` is an optional nanosecond precision device timestamp at which
    /// the device is to begin the transmission (c.f. [get_hardware_time](Device::get_hardware_time)).
    ///
    /// `end_burst` indicates when this packet ends a burst transmission.
    ///
    /// # Panics
    ///  * If `buffers` is not the same length as the `channels` array passed to `Device::rx_stream`.
    ///  * If all the buffers in `buffers` are not the same length.
    pub fn write_all(&mut self, buffers: &[&[E]], at_ns: Option<i64>, end_burst: bool, timeout_us: i64) -> Result<(), Error> {
        let mut buffers = buffers.to_owned();
        let mut at_ns = at_ns;

        while buffers.get(0).map_or(0, |x| x.len()) > 0 {
            // The timestamp is only sent on the first write.
            let written = self.write(&buffers, at_ns.take(), end_burst, timeout_us)?;

            // Advance the buffer pointers
            for buf in &mut buffers {
                *buf = &buf[written..];
            }
        }

        Ok(())
    }

    // TODO: read_status

    // TODO: DMA

}
