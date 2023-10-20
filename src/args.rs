use soapysdr_sys::*;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fmt;
use std::iter::{FromIterator, IntoIterator};
use std::os::raw::c_char;
use std::ptr;
use std::slice;

/// A list of key=value pairs.
pub struct Args(SoapySDRKwargs);

impl Drop for Args {
    fn drop(&mut self) {
        unsafe { SoapySDRKwargs_clear(self.as_raw()) }
    }
}

impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}

impl Args {
    /// Create a new, empty `Args` list
    pub fn new() -> Args {
        Args(SoapySDRKwargs {
            size: 0,
            keys: ptr::null_mut(),
            vals: ptr::null_mut(),
        })
    }

    /// # Safety
    ///
    /// Be careful that [`SoapySDRKwargs`] is either:
    /// - [`SoapySDRKwargs::keys`] and [`SoapySDRKwargs::vals`] are null and [`SoapySDRKwargs::size`] is 0 or
    /// - [`SoapySDRKwargs::keys`] and [`SoapySDRKwargs::vals`] both point to valid keys and vals of
    /// [`SoapySDRKwargs::size`] length.
    pub unsafe fn from_raw(a: SoapySDRKwargs) -> Args {
        Args(a)
    }

    pub fn as_raw(&mut self) -> *mut SoapySDRKwargs {
        &mut self.0 as *mut _
    }

    pub fn as_raw_const(&self) -> *const SoapySDRKwargs {
        &self.0 as *const _
    }

    fn keys(&self) -> &[*mut c_char] {
        unsafe { slice::from_raw_parts(self.0.keys, self.0.size) }
    }

    fn key(&self, idx: usize) -> &CStr {
        unsafe { CStr::from_ptr(self.keys()[idx]) }
    }

    fn values(&self) -> &[*mut c_char] {
        unsafe { slice::from_raw_parts(self.0.vals, self.0.size) }
    }

    fn value(&self, idx: usize) -> &CStr {
        unsafe { CStr::from_ptr(self.values()[idx]) }
    }

    /// Append a key-value pair to the arguments list
    ///
    /// # Example
    /// ```
    /// use soapysdr::Args;
    /// let mut args = Args::new();
    /// args.set("driver", "lime");
    /// ```
    ///
    /// # Panics
    ///  * if `key` or `value` contain null bytes
    pub fn set<K: Into<Vec<u8>>, V: Into<Vec<u8>>>(&mut self, key: K, value: V) {
        unsafe {
            let k = CString::new(key).expect("SoapySDR key can't contain null bytes");
            let v = CString::new(value).expect("SoapySDR value can't contain null bytes");
            SoapySDRKwargs_set(self.as_raw(), k.as_ptr(), v.as_ptr());
        }
    }

    /// Get the value corresponding to a key in the arguments list.
    ///
    /// ### Example:
    /// ```
    /// use soapysdr::Args;
    /// let args: Args = "serial=123456".into();
    /// assert_eq!(args.get("serial"), Some("123456"));
    /// ```
    pub fn get<'a>(&'a self, key: &str) -> Option<&'a str> {
        for i in 0..(self.0.size) {
            if self.key(i).to_bytes() == key.as_bytes() {
                return self.value(i).to_str().ok();
            }
        }
        None
    }

    /// Get an iterator over the (key, value) pairs in the arguments list.
    ///
    /// ### Example:
    /// ```
    /// use soapysdr::Args;
    /// let args: Args = "driver=lime, serial=123456".into();
    /// let mut i = args.iter();
    /// assert_eq!(i.next(), Some(("driver", "lime")));
    /// assert_eq!(i.next(), Some(("serial", "123456")));
    /// assert_eq!(i.next(), None);
    /// ```
    pub fn iter(&self) -> ArgsIterator {
        ArgsIterator { args: self, pos: 0 }
    }
}

impl<K: Into<Vec<u8>>, V: Into<Vec<u8>>> FromIterator<(K, V)> for Args {
    fn from_iter<T>(i: T) -> Self
    where
        T: IntoIterator<Item = (K, V)>,
    {
        let mut args = Args::new();
        for (k, v) in i {
            args.set(k, v);
        }
        args
    }
}

impl<'a> IntoIterator for &'a Args {
    type Item = (&'a str, &'a str);
    type IntoIter = ArgsIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> From<&'a str> for Args {
    fn from(s: &'a str) -> Args {
        let mut args = Args::new();
        for i in s.split(',') {
            if let Some(pos) = i.find('=') {
                args.set(i[..pos].trim(), i[pos + 1..].trim());
            }
        }
        args
    }
}

impl<'a, K: ::std::cmp::Eq + ::std::hash::Hash, V> From<&'a HashMap<K, V>> for Args
where
    &'a K: Into<Vec<u8>>,
    &'a V: Into<Vec<u8>>,
{
    fn from(m: &'a HashMap<K, V>) -> Args {
        let mut args = Args::new();
        for (k, v) in m {
            args.set(k, v);
        }
        args
    }
}

impl<'a, K, V> From<&'a [(K, V)]> for Args
where
    &'a K: Into<Vec<u8>>,
    &'a V: Into<Vec<u8>>,
{
    fn from(m: &'a [(K, V)]) -> Args {
        let mut args = Args::new();
        for (k, v) in m {
            args.set(k, v);
        }
        args
    }
}

impl From<()> for Args {
    fn from(_: ()) -> Args {
        Args::new()
    }
}

impl<'a> From<&'a Args> for String {
    fn from(a: &'a Args) -> String {
        format!("{}", a)
    }
}

impl<'a> From<&'a Args> for HashMap<String, String> {
    fn from(a: &'a Args) -> HashMap<String, String> {
        a.into_iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect()
    }
}

impl fmt::Display for Args {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut i = self.iter();
        if let Some((k, v)) = i.next() {
            write!(fmt, "{}={}", k, v)?;
            for (k, v) in i {
                write!(fmt, ", {}={}", k, v)?;
            }
        }
        Ok(())
    }
}

/// An iterator over the `(&key, &value)` pairs in an `Args` list.
pub struct ArgsIterator<'a> {
    args: &'a Args,
    pos: usize,
}

impl<'a> Iterator for ArgsIterator<'a> {
    type Item = (&'a str, &'a str);
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.args.0.size {
            let k = self.args.key(self.pos).to_str().unwrap_or("(invalid utf8)");
            let v = self
                .args
                .value(self.pos)
                .to_str()
                .unwrap_or("(invalid utf8)");
            self.pos += 1;
            Some((k, v))
        } else {
            None
        }
    }
}
