# Installation

This work is in the process of upstreaming into nightly Rust. In the future a manual installation should not be neccessary anymore (unless you want to contribute to this project). Till then, you can follow the instructions below. Please be aware that the msvc target is not supported at the moment, all other tier 1 targets should work. Please open an issue if you have issues on a supported tier 1 target, or if you succesfully build this project on a tier2/tier3 target.

## Build instructions

First you need to clone and configure this Rust fork:
```bash
git clone --depth=1 git@github.com:EnzymeAD/rust.git
cd rust
./configure --enable-llvm-link-shared --enable-llvm-plugins --enable-llvm-enzyme --release-channel=nightly --enable-llvm-assertions --enable-clang --enable-lld --enable-option-checking --enable-ninja --disable-docs
```

Afterwards you can build rustc using:
```bash
./x.py build --stage 1 library
```

Afterwards rustc toolchain link will allow you to use it through cargo:
```
rustup toolchain link enzyme build/host/stage1
rustup toolchain install nightly # enables -Z unstable-options
```

You can then run examples from our [docs](https://enzyme.mit.edu/index.fcgi/rust/usage/usage.html):

```bash
cd ..
git clone git@github.com:EnzymeAD/rustbook.git  
cd rustbook/samples
cargo +enzyme test reverse
```

If you want to use Autodiff in your own projects, you will need to add `lto="fat"` to your Cargo.toml 
and use `cargo +enzyme` instead of `cargo` or `cargo +nightly`. 


