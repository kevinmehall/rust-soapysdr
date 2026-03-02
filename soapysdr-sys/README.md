# Rust FFI for SoapySDR

[SoapySDR](https://github.com/pothosware/SoapySDR/wiki) is a hardware abstraction layer for many software defined radio devices.

This crate provides bindings for the SoapySDR C API, while the [soapysdr] crate provides a safe Rust wrapper.

[soapysdr]: https://crates.io/crates/soapysdr

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

## Updating bindings

The bindings were originally generated with bindgen, but subsequently hand-edited. To run bindgen again, use

```
rust-bindgen --rust-edition 2021 --distrust-clang-mangling --no-prepend-enum-name wrapper.h > /tmp/bindings.rs
diff -u /tmp/bindings.rs src/bindings.rs | egrep -v '^\+\s*///' | less
```

and copy over any new functions or changes.

## License

Boost Software License 1.0, (Same as SoapySDR itself)

See [LICENSE-BSL](./LICENSE-BSL) or http://opensource.org/licenses/BSL-1.0
