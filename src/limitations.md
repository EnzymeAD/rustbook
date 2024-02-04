# Current limitations

1) Enzyme currently does only support freestanding functions. We added some support for `self`, but don't link pieces together correctly in all cases. `self` does not exist on llvm-ir level, so it's just a matter of fixing our macro and should be easy to solve.
 
4) Parallelism: Enzyme currently does not handle Rust parallelism (rayon). Enzyme does (partly) support various parallel paradigms: OpenMP, MPI, CUDA, Rocm, Julia tasks. Enzyme only does need to support the lowest level of parallelism for each language, so adding support for Rust is not hard, but also not a high priority.



Enzyme does support custom allocators, but Rust-Enzyme does not expose support for it yet. Low priority.

TODO: Talk about the history of EnzymeAD and the status of the current implementation.  Talk about any current limitations and whether they might be lifted.  Talk about future possibilities and ongoing work.

TODO: web example: https://arbitrandomuser.github.io/thangs/freehandbezier/
