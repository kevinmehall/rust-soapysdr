# Rust bindings for SoapySDR

[SoapySDR](https://github.com/pothosware/SoapySDR/wiki) provides a hardware abstraction layer for many software defined radio devices.

**[API Documentation](https://docs.rs/soapysdr)** | **[crates.io](https://crates.io/crates/soapysdr)** | **[Changelog](https://github.com/kevinmehall/rust-soapysdr/releases)**

## Dependencies

This library requires dependencies not handled by Cargo:

- libsoapysdr 0.8.x
- pkg-config (Linux and macOS only)

### Ubuntu

(Tested on Ubuntu 24.04)

```console
sudo apt install libsoapysdr-dev pkg-config

# Choose the appropriate drivers for your hardware:
sudo apt install soapysdr-module-rtlsdr soapysdr-module-hackrf soapysdr-module-uhd soapysdr-module-lms7
```

### Nix

`soapysdr-with-plugins` and `pkg-config`

(see [shell.nix](https://github.com/kevinmehall/rust-soapysdr/blob/master/shell.nix))

### Windows

Install [pre-built PothosSDR] and add PothosSDR bin directory to system `PATH`.

[pre-built PothosSDR]: https://github.com/pothosware/PothosSDR/wiki/Tutorial

### MacOS

Install SoapySDR with Homebrew:

```
brew install pkg-config
brew tap pothosware/homebrew-pothos
brew update

# Then install the appropriate packages for your hardware:
brew install soapyrtlsdr
brew install soapyhackrf
brew install soapybladerf
# ...
```

## Warning

Many SoapySDR driver modules have error handling and thread safety bugs. This library provides
safe Rust wrappers assuming the drivers meet the (under-documented) intentions of the SoapySDR
core API contract, but if SoapySDR loads modules that violate this contract and you do atypical
things with them, you may encounter unexpected behavior. For details, see
[this SoapySDR issue](https://github.com/pothosware/SoapySDR/issues/111).

## Examples

This crate comes with two small utilities that serve as example code.

### soapy-sdr-info

Displays device details like `SoapySDRUtil`.

```
cargo run --release --example soapy-sdr-info
```

### soapy-sdr-stream

Records data from a device.

e.g. capture 15 seconds of data from the FM band:

```
cargo run --release --example soapy-sdr-stream -- -d driver=rtlsdr -r out.cfile -f 96M -s 1M -n 15M
```

The resulting file contains 32-bit little-endian complex float samples, and can be opened with
[inspectrum](https://github.com/miek/inspectrum).

## License

Licensed under either of

- Apache License, Version 2.0, (LICENSE or http://www.apache.org/licenses/LICENSE-2.0)
- Boost Software License 1.0, (Same as SoapySDR itself, LICENSE-BSL or http://opensource.org/licenses/BSL-1.0)
