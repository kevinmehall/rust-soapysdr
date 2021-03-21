# Rust bindings for SoapySDR

[SoapySDR](https://github.com/pothosware/SoapySDR/wiki) provides a hardware abstraction layer for many software defined radio devices.

**[API Documentation](https://kevinmehall.net/rustdoc/soapysdr/soapysdr/)** | **[Changelog](https://github.com/kevinmehall/rust-soapysdr/releases)**

## Dependencies

This library requires dependencies not handled by Cargo:

  * libsoapysdr 0.6, 0.7, or git master (0.8)
  * libclang 3.9+ (for bindgen)

### Ubuntu

(Tested on Ubuntu 20.04)

```console
sudo apt install libsoapysdr-dev libclang-dev llvm-dev pkg-config

# Choose the appropriate drivers for your hardware:
sudo apt install soapysdr-module-rtlsdr soapysdr-module-hackrf soapysdr-module-uhd soapysdr-module-lms7
```

### Nix

```
nix-shell
```

(see [shell.nix](./shell.nix))

## Warning

Many SoapySDR driver modules have error handling and thread safety bugs. This library provides
safe Rust wrappers assuming the drivers meet the (under-documented) intentions of the SoapySDR
core API contract, but if SoapySDR loads modules that violate this contract and you do atypical
things with them, you may encounter unexpected behavior. For details, see
[this SoapySDR issue](https://github.com/pothosware/SoapySDR/issues/111).

## Utilities

This crate comes with two small utilities that serve as example code.

### soapy-sdr-info

Displays device details like `SoapySDRUtil`.

```
cargo run --release --features=binaries --bin soapy-sdr-info
```

### soapy-sdr-stream

Records data from a device.

e.g. capture 15 seconds of data from the FM band:

```
cargo run --release --features=binaries --bin soapy-sdr-stream -- -d driver=rtlsdr -r out.cfile -f 96M -s 1M -n 15M
```

The resulting file contains 32-bit little-endian complex float samples, and can be opened with
[inspectrum](https://github.com/miek/inspectrum).

## License

Licensed under either of

  - Apache License, Version 2.0, (LICENSE or http://www.apache.org/licenses/LICENSE-2.0)
  - Boost Software License 1.0, (Same as SoapySDR itself, LICENSE-BSL or http://opensource.org/licenses/BSL-1.0)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
