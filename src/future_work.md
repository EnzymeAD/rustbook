# Future Work


### Parallelism: 
Enzyme currently does not handle Rust parallelism (rayon). 
Enzyme does (partly) support various parallel paradigms: OpenMP, MPI, CUDA, Rocm, Julia tasks. 
Enzyme only does need to support the lowest level of parallelism for each language, 
so adding support for rayon should cover most cases. We assume 20-200 lines of code in 
Enzyme core should be sufficient, making it a nice task to get started.  
[rsmpi](https://github.com/rsmpi/rsmpi) (Rust wrapper for MPI) should already work, but it would be good to test.

### Batching
Batching allows computing multiple derivatives at once. This can help amortizing the cost
of the forward pass. It can also be used to enable future vectorization. This feature
is quite popular for ML projects. The JAX documentation gives an example [here](https://jax.readthedocs.io/en/latest/jax-101/04-advanced-autodiff.html#per-example-gradients).
Batching is supported by Enzyme core and can be trivially implemented for Rust-Enzyme in a few hours, 
the main blocker is bikesheding are around the frontend. Do we want to accept `N` individual shadow arguments?
Do we want to accept a tuple of N elements? An array `[T;N]`?


### Custom Derivatives
Let's assume that you want to use [differentiable rendering](https://arxiv.org/abs/2006.12057), 
but someone added a "fast" version of the [inverse square root function](https://en.wikipedia.org/wiki/Fast_inverse_square_root#Overview_of_the_code) to your render engine, 
breaking your Rust-Enzyme tool, which can't figure out how `i  = 0x5f3759df - ( i >> 1 );` would affect your gradient. 
AutoDiff packages for this reason allow declaring a custom derivative `f'` for a function `f`. 
In such a case the AD tool will not look at the implementation of `f` and directly use the user provided `f'`. 
Jax documentation also has a large list of other reasons due to which you might want to use custom derivatives: [link](https://jax.readthedocs.io/en/latest/notebooks/Custom_derivative_rules_for_Python_code.html).
Julia has a whole ecosystem called [ChainRules.jl](https://juliadiff.org/ChainRulesCore.jl/stable/) around custom derivatives. 
Enzyme does support custom derivatives, but we do not expose this feature on the Rust side yet.
Together with the Batching features, this is one of the highest rewards / lowest effort improvements planed for Rust-Enzyme.


### Custom Allocators: 
Enzyme does support custom allocators, but Rust-Enzyme does not expose support for it yet. 
Please let us know if you have an application that can benefit from a custom allocator and autodiff,
otherwise this likely won't be implemented in the forseeable future.

### Checkpointing:
While Enzyme is very fast due to running optimizations before AD, we don't explore all the classical AutoDiff tricks yet. Namely we do miss support for adjusting checkpointing decisions, which describes the question of whether we want to cache or recompute values needed for the gradient computations. It generally lies in NP to find the optimal balance for each given program, but there are good approximations. You can think of it in terms of custom allocators. Replacing the algorithm might affect your runtime performance, but does not affect the result of your function calls. In the future it might be interesting to let the user interact with checkpointing.

### Supporting other Codegen backends:
Enzyme core consists of ~50k LoC. Most of the rules around generating derivatives for instructions are written in LLVM Tablegen.td declarations and as such it should be relatively easy to port them. Enzyme core also includes various experimental features which we don't need on the Rust side, an implementation for another codegen backend could therefore also end up a bit smaller.
The cranelift backend would also benefit from ABI compability, which makes it very easy to test correctness of a new autodiff tool against Enzyme. Our modifications to `rustc_codegen_ssa` and previous layers of rustc are written in a generic way, s.t. no changes would be needed there to enable support for additional backends.

### GPU / TPU / IPU / ... support.
Enzyme core supports differentiating CUDA/ROCm Kernels. 
There are various ways towards exposing this capabilities to Rust.
Manuel and Jed will be experimenting with two different approaches in 2024,
and there is also a lot of simultaneous research. Please reach out if 
you are also working on GPU programming in Rust.

### MLIR support:
Enzyme partly supports multiple MLIR dialects. MLIR can offer great runtime
performance benefits for certain workloads. It would be nice to have a 
`rustc_codegen_mlir`, but there is a very large number of open questions around the design.
