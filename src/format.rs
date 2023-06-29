use std::str::FromStr;
use std::os::raw::c_char;
use num_complex::Complex;
use soapysdr_sys::*;

/// Data format of samples
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Format {
    /// Complex 64-bit floats (complex double)
    CF64,

    /// Complex 32-bit floats (complex float)
    CF32,

    /// Complex signed 32-bit integers (complex int32)
    CS32,

    /// Complex unsigned 32-bit integers (complex uint32)
    CU32,

    /// Complex signed 16-bit integers (complex int16)
    CS16,

    /// Complex unsigned 16-bit integers (complex uint16)
    CU16,

    /// Complex signed 12-bit integers (3 bytes)
    CS12,

    /// Complex unsigned 12-bit integers (3 bytes)
    CU12,

    /// Complex signed 8-bit integers (complex int8)
    CS8,

    /// Complex unsigned 8-bit integers (complex uint8)
    CU8,

    /// Complex signed 4-bit integers (1 byte)
    CS4,

    /// Complex unsigned 4-bit integers (1 byte)
    CU4,

    /// Real 64-bit floats (double)
    F64,

    /// Real 32-bit floats (float)
    F32,

    /// Real signed 32-bit integers (int32)
    S32,

    /// Real unsigned 32-bit integers (uint32)
    U32,

    /// Real signed 16-bit integers (int16)
    S16,

    /// Real unsigned 16-bit integers (uint16)
    U16,

    /// Real signed 8-bit integers (int8)
    S8,

    /// Real unsigned 8-bit integers (uint8)
    U8,
}

type ParseFormatError = ();

impl FromStr for Format {
    type Err = ParseFormatError;

    fn from_str(s: &str) -> Result<Format, ParseFormatError> {
        match s {
            "CF64" => Ok(Format::CF64),
            "CF32" => Ok(Format::CF32),
            "CS32" => Ok(Format::CS32),
            "CU32" => Ok(Format::CU32),
            "CS16" => Ok(Format::CS16),
            "CU16" => Ok(Format::CU16),
            "CS12" => Ok(Format::CS12),
            "CU12" => Ok(Format::CU12),
            "CS8" => Ok(Format::CS8),
            "CU8" => Ok(Format::CU8),
            "CS4" => Ok(Format::CS4),
            "CU4" => Ok(Format::CU4),
            "F64" => Ok(Format::F64),
            "F32" => Ok(Format::F32),
            "S32" => Ok(Format::S32),
            "U32" => Ok(Format::U32),
            "S16" => Ok(Format::S16),
            "U16" => Ok(Format::U16),
            "S8" => Ok(Format::S8),
            "U8" => Ok(Format::U8),
            _ => Err(()),
        }
    }
}

impl Format {
    fn as_str_with_nul(&self) -> &'static str {
        match *self {
            Format::CF64 => "CF64\0",
            Format::CF32 => "CF32\0",
            Format::CS32 => "CS32\0",
            Format::CU32 => "CU32\0",
            Format::CS16 => "CS16\0",
            Format::CU16 => "CU16\0",
            Format::CS12 => "CS12\0",
            Format::CU12 => "CU12\0",
            Format::CS8 => "CS8\0",
            Format::CU8 => "CU8\0",
            Format::CS4 => "CS4\0",
            Format::CU4 => "CU4\0",
            Format::F64 => "F64\0",
            Format::F32 => "F32\0",
            Format::S32 => "S32\0",
            Format::U32 => "U32\0",
            Format::S16 => "S16\0",
            Format::U16 => "U16\0",
            Format::S8 => "S8\0",
            Format::U8 => "U8\0",
        }
    }

    pub(crate) fn as_ptr(&self) -> *const c_char {
        self.as_str_with_nul().as_ptr() as *const c_char
    }

    /// Get the name of the format
    pub fn as_str(&self) -> &str {
        let s = self.as_str_with_nul();
        &s[..s.len()-1]
    }

    /// Get the size of one sample in this format
    pub fn size(&self) -> usize {
        unsafe { SoapySDR_formatToSize(self.as_ptr()) }
    }
}

impl ::std::fmt::Debug for Format {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ::std::fmt::Display for Format {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Trait for sample formats used by a TxStream or RxStream
///
/// # Safety
///
/// Implementing this trait requires that the type have the same size, alignment, and compatible
/// memory representation with the SoapySDR type selected by `STREAM_FORMAT`
pub unsafe trait StreamSample {
    const STREAM_FORMAT: Format;
}

unsafe impl StreamSample for u8           { const STREAM_FORMAT: Format = Format::U8; }
unsafe impl StreamSample for u16          { const STREAM_FORMAT: Format = Format::U16; }
unsafe impl StreamSample for u32          { const STREAM_FORMAT: Format = Format::U32; }
unsafe impl StreamSample for i8           { const STREAM_FORMAT: Format = Format::S8; }
unsafe impl StreamSample for i16          { const STREAM_FORMAT: Format = Format::S16; }
unsafe impl StreamSample for i32          { const STREAM_FORMAT: Format = Format::S32; }
unsafe impl StreamSample for f32          { const STREAM_FORMAT: Format = Format::F32; }
unsafe impl StreamSample for f64          { const STREAM_FORMAT: Format = Format::F64; }
//unsupported CU4
unsafe impl StreamSample for Complex<u8>  { const STREAM_FORMAT: Format = Format::CU8; }
//unsupported CU12
unsafe impl StreamSample for Complex<u16> { const STREAM_FORMAT: Format = Format::CU16; }
unsafe impl StreamSample for Complex<u32> { const STREAM_FORMAT: Format = Format::CU32; }
//unsupported CS4
unsafe impl StreamSample for Complex<i8>  { const STREAM_FORMAT: Format = Format::CS8; }
//unsupported CS12
unsafe impl StreamSample for Complex<i16> { const STREAM_FORMAT: Format = Format::CS16; }
unsafe impl StreamSample for Complex<i32> { const STREAM_FORMAT: Format = Format::CS32; }
unsafe impl StreamSample for Complex<f32> { const STREAM_FORMAT: Format = Format::CF32; }
unsafe impl StreamSample for Complex<f64> { const STREAM_FORMAT: Format = Format::CF64; }
