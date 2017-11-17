# Rust bindings for SoapySDR

[SoapySDR](https://github.com/pothosware/SoapySDR/wiki) provides a hardware abstraction layer for many software defined radio devices.

**[API Documentation](https://kevinmehall.net/rustdoc/soapysdr/soapysdr/)**

## Dependencies

This library requires libsoapysdr 0.6.0 and libclang 3.5+ (for bindgen) to be installed manually.

### Ubuntu

(Tested on Ubuntu 16.04)

```console
sudo add-apt-repository ppa:myriadrf/drivers
sudo apt update
sudo apt install libsoapysdr-dev llvm-3.9-dev libclang-3.9-dev

# Choose the appropriate drivers for your hardware:
sudo apt install soapysdr-module-rtlsdr soapysdr-module-hackrf soapysdr-module-uhd soapysdr-module-lms7
```

### Warning

Many SoapySDR driver modules have error handling and thread safety bugs. This library provides
safe Rust wrappers assuming the drivers meet the (under-documented) intentions of the SoapySDR
core API contract, but if SoapySDR loads modules that violate this contract and you do atypical things with them, you may encounter unexpected behavior. For details, see
[this SoapySDR issue](https://github.com/pothosware/SoapySDR/issues/111).

## License

Licensed under either of

  - Apache License, Version 2.0, (LICENSE or http://www.apache.org/licenses/LICENSE-2.0)
  - Boost Software License 1.0, (Same as SoapySDR itself, LICENSE-BSL or http://opensource.org/licenses/BSL-1.0)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
