# How to Debug AD?

Since Rust-AD is still in early development, crashes are not unlikely.

### Frontend crashes
If you see a proper Rust stacktrace after a compilation failure, our frontend (thus rustc) has likely crashed.
It should often be trivial to create a minimal reproducer, by deleting most of the body of the 
function being differentiated, or by replacing the function body with a `loop {}` statement.
Please create an issue with such a reproducer, it will likely be easy to fix!

For the unexpected case, that you produce an ICE in our frontend that 
is harder to minimize, please consider using [icemelter](https://github.com/langston-barrett/icemelter).

### Backend crashes 
If you see llvm-ir (a language which might remind you of assembly), then our backend crahed. 
You can find instructions on how to create an issue and help us to fix it [on the next page](debug_backend.md).

### Debuging and Profiling 
Rust-AD supports passing an `autodiff` flag to `RUSTFLAGS`, which supports changing the behaviour of Enzyme in various ways.
Documentation is availabile [here](debug_flags.md).
