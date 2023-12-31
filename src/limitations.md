# Current limitations

1) Enzyme currently does only support freestanding functions. We added some support for `self`, but don't link pieces together correctly in all cases. `self` does not exist on llvm-ir level, so it's just a matter of fixing our macro and should be easy to solve.

2) Soundness: Enzyme currently does assume that the user passes shadow arguments (`dx`, `dy`, ...) of appropriate size. That's a current research project of Manuel, so we hope to check at least basic DST (vectors, enums) till end of November. If we remember the backprop function from above, there is no way for the type system to guarantee that `dweights` is at least as large as the `weights` vector. Adding length checks for vectors and making sure that in case of enums primal and shadow are of the same variant should get us a large step towards soundness. Once implemented, we can still evaluate how many more checks we can insert automatically and where we want to fall back to unsafe. We also consider to allow the user to (unsafely) implement a safety check for his own types which we would then insert. Concretely, here we would add the following check at the top of backprop (above the code generated by enzyme) `assert!(dweights.len() >= weights.len())`
```rust
fn backprop(images: &[f32], weights: &[f32], dweights: &mut [f32]) { ... }
```
    
3) Computing higher order derivatives (hessians, ...) can be done with Enzyme by differentiating functions that compute lower order derivatives. [This example](https://github.com/EnzymeAD/rust/blob/master/library/autodiff/examples/hessian_sin.rs) requires that rustc first uses Enzyme to fill the implementation of the `jac` function, before it uses Enzyme to fill the implementation of `hess`, by differentiatng `jac`. This is currently not guaranteed. It should be comparably easy to fix. Enzyme also considers adding helper function to directly compute common higher order derivatives.


4) Parallelism: Enzyme currently does not handle Rust parallelism (rayon). Enzyme does (partly) support various parallel paradigms: OpenMP, MPI, CUDA, Rocm, Julia tasks. Enzyme only does need to support the lowest level of parallelism for each language, so adding support for Rust is not hard, but also not a high priority.


5) Compile Times: Enzyme can create the TypeTrees it requires for each variable based on its LLVM-IR reads/writes/dereferences/usages. This is slow for large types and programms. A (real-world) examples which updates a vector of length 50k element by element, line by line e.g. takes 6hrs to compile with C++ Enzyme due to TypeTree creation, while JAX only takes 1hr. Due to leveraging rustc information (see next point) Rust-Enzyme manages to compile the code in 5 minutes. Compiling the C++ code without AD however only takes ~1 second.

6) Rust ABI optimizations: In order to improve the compile times mentioned above we create Enzyme TypeTrees based on the rustc type knowledge. These trees unfortunately are not correct anymore once Rust ABI optimization take place. E.g. `fn rosenbrock(x: &[f64; 2]) -> f64 {...}` will get lowered into an LLVM-IR function comparable to `fn rosenbrock(f64, f64) -> f64`. A TypeTree therefore needs to be updated on each applied optimization. For now, we just block optimizations on the outermost functions since they tend to have a small performance effect, so we want to focus on other parts first.

7) FAT-LTO requirement: Rust-Enzyme currently requires fat-lto when AutoDiff is used. We technically only need lto if the function being differentiated calls functions in other compilation units. Other solutions are possible but this is the most simple one. Since the compile time overhead of lto is small compared to the compile time overhead of differentiating larger functions this is not a priority.

Enzyme does support custom allocators, but Rust-Enzyme does not expose support for it yet. Low priority.

TODO: Talk about the history of EnzymeAD and the status of the current implementation.  Talk about any current limitations and whether they might be lifted.  Talk about future possibilities and ongoing work.

TODO: web example: https://arbitrandomuser.github.io/thangs/freehandbezier/
