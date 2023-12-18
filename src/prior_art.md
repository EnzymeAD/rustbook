# Prior art

Autodiff emerged as an identified mathematical framework and software tool in the 1980s, building on groundwork from previous decades. The common implementation strategies were operator overloading and source transformation. In the former approach, one replaces scalars with a pair carrying the primary and variation and overloads elementary operations, leading to code similar to the following:

```rust
struct ActiveReal(f64, f64);

impl Add for ActiveReal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul for ActiveReal {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        // Evoke the product rule, d(xy) = x * dy + dx * y
        Self(self.0 * rhs.0, self.0 * rhs.1 + self.1 * rhs.0)
    }
}
```
This approach can be distributed in a library (in languages that support operator overloading), but in practice, it means making most user functions generic and has serious performance impacts. For example, loops are effectively unrolled and this strategy is harmful to vectorization since "even and odd lanes" require different processing. Some libraries, especially in C++, have applied heavy template metaprogramming to make those libraries reasonably performant, but they still lag behind and have obtuse error messages.

The second approach, source transformation, was first applied to Fortran and later C, but never differentiated the complete language. Those systems implemented their own parsers and would generate new source files with mangled symbols and calling conventions, to be manually included by the build systems. Such approaches could be much more efficient by retaining loop structure and enabling vectorization, but the language coverage was always incomplete and they have historically been difficult to install (e.g., OpenAD depended on the research compiler ROSE, which could only be compiled with a specific version of gcc). Error handling and debugging is also poor with this approach.

Autodiff remained an active research area with relatively clumsy research-grade tooling for decades. Select scientific applications, such as MITgcm, would shape their entire software engineering around enabling source transformation tools to compute derivatives. The most recent machine learning boom was precipitated by autodiff engineering improving to the point where practitioners could rapidly design new neural network architectures and have efficient gradients automatically available for gradient-based optimization. The modern deep learning libraries (PyTorch, TensorFlow, JAX) have grown into more general autodiff tools, albeit with language restrictions and [sharp edges](https://jax.readthedocs.io/en/latest/notebooks/Common_Gotchas_in_JAX.html#control-flow) due to custom JIT compilation separate from their host language (Python with C++ infrastructure).

Enzyme takes advantage of the fact that while differentiating all features of the source language is a monumental task, differentating a single-statement assignment (SSA) form such as LLVM IR is far more tractable and allows complete language coverage. Since it's based on LLVM, it is language-agnostic for all LLVM-based languages.

TODO: https://enzyme.mit.edu/conference/assets/JanHueckelheim_EnzymeCon2023.pdf [[paper](https://arxiv.org/abs/2305.07546)]

TODO: Talk about Torch, TensorFlow, PyTorch, JAX, Julia, etc.

