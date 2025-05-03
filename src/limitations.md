# Current limitations

[Current limitations](https://rustc-dev-guide.rust-lang.org/autodiff/limitations.html) are tracked in the rustc-dev-guide.


# Future Work


### Parallelism: 
Enzyme supports the ability to efficiently differentiate parallel code. Enzyme's unique ability to combine optimization (including parallel optimization) enables orders of magnitude improvements on performance and [scaling parallel code](https://ieeexplore.ieee.org/document/10046093). Each parallel framework needs only provide Enzyme lightweight markers describing where the parallelism is created (e.g. this is a parallel for or spawn/sync). Such markers have been added for various parallel paradigms, including: CUDA, ROCm, OpenMP, MPI, Julia tasks, and RAJA.

Such markers have not been added for Rust parallel libraries (i.e. rayon). Enzyme only does need to support the lowest level of parallelism for each language, 
so adding support for rayon should cover most cases. We assume 20-200 lines of code in 
Enzyme core should be sufficient, making it a nice task to get started.  
[rsmpi](https://github.com/rsmpi/rsmpi) (Rust wrapper for MPI) should already work, but it would be good to test.

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
While Enzyme is very fast due to running optimizations before AD, including various partial checkpointing algorithms -- such as a [min-cut algorithm](https://dl.acm.org/doi/abs/10.1145/3458817.3476165). The ability to control checkpointing (e.g. whether to recompute or store) has not yet been added to Rust. Optimal checkpointing generally lies in NP to find the optimal balance for each given program, but there are good approximations. You can think of it in terms of custom allocators. Replacing the algorithm might affect your runtime performance, but does not affect the result of your function calls. In the future it might be interesting to let the user interact with checkpointing.

### Supporting other Codegen backends:
Enzyme consists of ~50k LoC. Most of the rules around generating derivatives for instructions are written in LLVM Tablegen.td declarations and as such it should be relatively easy to port them. Enzyme also includes various experimental features which we don't need on the Rust side, an implementation for another codegen backend could therefore also end up a bit smaller.
The cranelift backend would also benefit from ABI compatibility, which makes it very easy to test correctness of a new autodiff tool against Enzyme. Our modifications to `rustc_codegen_ssa` and previous layers of rustc are written in a generic way, s.t. no changes would be needed there to enable support for additional backends.

### GPU / TPU / IPU / ... support.
Enzyme supports differentiating CUDA/ROCm Kernels. 
There are various ways towards exposing this capabilities to Rust.
Manuel and Jed will be experimenting with two different approaches in 2024,
and there is also a lot of simultaneous research. Please reach out if 
you are also working on GPU programming in Rust.

### MLIR support:
Enzyme partly supports multiple MLIR dialects. MLIR can offer great runtime
performance benefits for certain workloads. It would be nice to have a 
`rustc_codegen_mlir`, but there is a very large number of open questions around the design.
