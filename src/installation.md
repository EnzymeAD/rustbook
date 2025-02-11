# Installation

This work is in the process of upstreaming into nightly Rust. In the future a manual installation should not be neccessary anymore (unless you want to contribute to this project). Till then, you can follow the instructions below. Please be aware that the msvc target is not supported at the moment, all other tier 1 targets should work. Please open an issue if you have issues on a supported tier 1 target, or if you succesfully build this project on a tier2/tier3 target.

## Build instructions

First you need to clone and configure this Rust fork:
```bash
git clone --depth=1 git@github.com:rust-lang/rust.git
cd rust
./configure --enable-llvm-link-shared --enable-llvm-plugins --enable-llvm-enzyme --release-channel=nightly --enable-llvm-assertions --enable-clang --enable-lld --enable-option-checking --enable-ninja --disable-docs
```
PSA: 
At the time of writing, the enzyme feature on the main branch is slightly broken, we hope to fix this over the next week or two.
You should consider cloning [this branch](https://github.com/rust-lang/rust/pull/136428), as it has a one-line fix, needed to build rust-enzyme.

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

## Compiler Explorer and dist builds

Our compiler explorer instance can be updated to a newer rustc in a similar way. First, prepare a docker instance.
```bash
docker run -it ubuntu:22.04
export CC=clang CXX=clang++
apt update
apt install wget vim python3 git curl libssl-dev pkg-config lld ninja-build cmake clang build-essential 
```
Then build rustc in a slightly altered way:
```bash
git clone --depth=1 https://github.com/EnzymeAD/rust.git
cd rust
./configure --enable-llvm-link-shared --enable-llvm-plugins --enable-llvm-enzyme --release-channel=nightly --enable-llvm-assertions --enable-clang --enable-lld --enable-option-checking --enable-ninja --disable-docs
./x dist
```
We then copy the tarball to our host. The dockerid is the newest entry under `docker ps -a`.
```bash
docker cp <dockerid>:/rust/build/dist/rust-nightly-x86_64-unknown-linux-gnu.tar.gz rust-nightly-x86_64-unknown-linux-gnu.tar.gz
```
Afterwards we can create a new (pre-release) tag on the EnzymeAD/rust repository and make a PR against the EnzymeAD/enzyme-explorer repository to update the tag.
Remember to ping `tgymnich` on the PR to run his update script.

