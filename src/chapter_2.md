# Chapter 1

---
title: "Design document: Autodiff"
date: 2023-10-28
tags: design-document
url: https://hackmd.io/N27wUWGmTGa0RyYOSEi8bA
---

# Summary

We propose to add automatic differentiation to Rust.  This would allow Rust users to compute the derivatives of arbitrary functions.  This feature would open new opportunities for Rust in scientific computing, machine learning, and other fields.

# Background

## What is autodiff?

Automatic/algorithmic differentiation, or autodiff, is...

TODO with examples.

## What is autodiff used for?

TODO with examples.  What kind of problems does this solve that nothing else solves?

Autodiff is widely used to evaluate gradients for numerical optimization, which is otherwise intractable for a large number of parameters. Indeed, suppose we have a scalar-valued loss function $f(\theta)$ where the parameter vector $\theta$ has length $n$. If the cost to evaluate $f(x)$ once is $c$ (which will often be $O(n)$), then evaluating the gradient $\partial f/\partial x$ costs less than $3n$ with autodiff or tedious and brittle by-hand implementation, but $cn$ otherwise. Optimization of systems of size $n$ in the hundreds to billions are common in applications such as calibration, data assimilation, and design optimization of physical models, in perception and control systems for robotics, and machine learning.

Derivatives are also instrumental to thermodynamically admissible physical models, in which models are developed using non-observable free energy functionals and dissipation potentials, with observable dynamics represented by their derivatives. Commercial engineering software requires users to implement these derivatives by hand (e.g., Abaqus [`UHYPER`](https://abaqus-docs.mit.edu/2017/English/SIMACAESUBRefMap/simasub-c-uhyper.htm#simasub-c-uhyper-t-vartodefine1) and [`UMAT`](https://abaqus-docs.mit.edu/2017/English/SIMACAESUBRefMap/simasub-c-umat.htm#simasub-c-umat-t-vartodefine1)) and constitutive modeling papers routinely spend many pages detailing how to efficiently compute the necessary derivatives since these are among the most computationally intensive parts of simulation-based workflows and numerical stability is necessary.

## Prior art

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

# Solution

## Autodiff on LLVM IR



TODO: Typical LICM $O(n)$ vs $O(n^2)$ Enzyme example.
TODO: Talk about what makes this approach special and a good fit for Rust conceptually.

## Changes to Rust

TODO: Talk about the new attributes and define the semantics of these new attributes.  Give examples.

### Design:
For all practical purposes, AutoDiff consists here of two functions and a few config settings.
We have one primal, user written function which we'll call `f`. AutoDiff will give us (based on a certain config) a new function `df`, which we can call to compute our selected gradients. When can therefore provide Enzyme/rustc with the required config either at the call-site of `df`, at the definition of `f`, or at the definition side of`df`. The call site usage can be solved by a normal rust macro which would expand to a declaration side macro and a direct call to that newly declared function on the next line. There are cases where we want to differentiate the same function multiple times with respect to different parameters (e.g. we optimize parameters in a simulation independently). In this case we don't want to have a single function `df`, but rather multiple functions `dfdx`, `dfdy`, `dfdz`, .... Our macro accounts for that and can be applied multiple times to `f`.

The definition of `df` here is special. Since it is inherently tied to the definition of `f` we need to prevent the user from modifying it, which we can use by only allowing autodiff access through our `#[autodiff]` macro. We will then add high-level Rust checks and extend `df` once compiled down to llvm-ir by the enzyme generated code for gradient computations.

Let's look at the configuration now:
1) The first argument of our macro is the name (or path) to the "other" function. That is, if we apply our macro to `f`, then we would put the name of `df`, which the macro will then generate. If we put our macro on top of an empty function, then the name should be the one of the function which we want to differentiate.
2) Forward vs Reverse mode: From a math point of view, this one describes in which order we will expand the chain rule, or rather if we want to calculate the vector-jacobian product, or the jacobian-vector product. It has a very large impact on the performance and the following config parameters will differ slightly based on the mode. No real impact beside of performance. Technically something in between Forward and Reverse would be possible, but has not been explored much by Jax, Enzyme, or others.
3) Activity of arguments, in the order in which they appear in `f`. If we use ReverseMode and return a non `()` value, we specify activity for the return too. We are open to discuss the actual names here:`Const`, `Active`, `Duplicated`, `DuplicatedNoNeed`.

#### Forward mode
WIP

In Forward mode we are only allowed to mark input arguments 
The return value of forward mode with a Duplicated return is a tuple containing as the first value the primal return value and as the second value the derivative.

In forward mode Duplicated(x, 0.0) is equivalent to Const(x), except that we can perform more optimizations for Const.


#### Reverse Mode
Both the inplace and "normal" variant return the gradient. The difference is that with Active the gradient is returned and with Duplicated the gradient is accumulated in place.




### Usage story
Let us start by looking at the most basic examples we can think of:

$f(x,y) = x^2 + y*3$

We have two input variables $x$, $y$ and a scalar return value.  Just to check our sanity, we first pass it to [wolfram alpha](https://www.wolframalpha.com/input?i2d=true&i=D%5BPower%5Bx%2C2%5D+%2B+y*3%2Cx%5D%3B+D%5BPower%5Bx%2C2%5D+%2B+y*3%2Cy%5D%3B+). No big surprises so far. Let's check for Enzyme (our compiler explorer does not handle Rust yet, so you'l have to trust me on this).

```rust
#[autodiff(df, Reverse, Active, Active, Active)]
fn f(x: f32, y: f32) -> f32 {
  x*x + 3.0 * y
}
```

Enzyme actually generates the code on LLVM-IR level, but Rust is nicer to read, so I will pretend we would generate a Rust implementation:

```rust
fn f(x: f32, y: f32) -> f32 {
  x*x + 3.0 * y
}
fn df(x: f32, y: f32) -> (f32, f32, f32) {
  (2.0 * x, 3.0, x*x + 3.0 * y)
}
```

$x*x$ becomes $2.0*x$, while $3.0 * y$ becomes $3.0$. The last argument is our original return value. However, we don't always pass things by value, so let's make sure we have a sensible solution:

```rust
#[autodiff(df, Reverse, Active, Duplicated, Active)]
fn f(x: f32, y: &f32) -> f32 {
  x*x + 3.0 * y
}
```

(pay attention to y).

```rust
fn f(x: f32, y: f32) -> f32 {
  x*x + 3.0 * y
}
fn df(x: f32, y: &f32, dy: &mut f32) -> (f32, f32) {
  dy += 3.0;
  (2.0 * x, x*x + 3.0 * y)
}
```

In the case of references (or pointers) we do expect the user to create `dy`.

We could obviously zero-initialize a float for the user, but let's assume the constructor is complex due to involving a double-linked-list or ffi, so we can't guarantee that on the compiler side. As an alternative motivation, imagine that we call `df` 5 times in a loop. It is clear that in this case the accumulated gradients should be 5 times higher too, which won't happen if we set `dy = 3.0` each time, instead of using `+=`. Yet another reason would be higher-order derivatives (todo: just refer to literature?).

Now that we got back from this rabbit hole, let's go wild and train a neural network on our local national lab server:

```rust
#[autodiff(backprop, Reverse, Duplicated, Duplicated, Active)]
fn inference(images: &[f32], weights: &[f32]) -> f32 {
  let loss = do_some_math(images, weights);
  loss
}
```

Now Enzyme gives us:

```rust
#[autodiff(backprop, Reverse, Duplicated, Duplicated, Active)]
fn inference(images: &[f32], weights: &[f32]) -> f32 {
  let loss = do_some_math(images, weights);
  loss
}
fn backprop(images: &[f32], dimages: &mut [f32], weights: &[f32], dweights: &mut [f32]) -> f32 {
  enzyme_update_inplace_dx(dimages);
  enzyme_update_inplace_dy(dweights);
  let loss = do_some_math(images, weights);
  loss
}
```

_Uuuuhm. Yeah?_ We want to minimize our loss, so let's do `weights -= learning_rate * dweights;`

We also just learned how we can update our images through `dimages`, but unless you know how to shape the world around you that's pretty useless, so we just wasted a good amount of our compute time for not needed gradients. Let's try again:

```rust
#[autodiff(backprop, Reverse, Constant, Duplicated, Active)]
fn inference(images: &[f32], weights: &[f32]) -> f32 {
  let loss = do_some_math(images, weights);
  loss
}
```

After all, we shouldn't modify our train and test images to improve our accuracy, right? So we now generate:

```rust
fn inference(images: &[f32], weights: &[f32]) -> f32 {
  let loss = do_some_math(images, weights);
  loss
}
fn backprop(images: &[f32], weights: &[f32], dweights: &mut [f32]) {
  enzyme_update_inplace_dy(dweights);
  let loss = do_some_math(x,y);
  loss
}
```

Great. No more random dimages that we don't know how to handle. Perfection? Almost:

```rust
#[autodiff(backprop, Reverse, Constant, Duplicated, DuplicatedNoNeed)]
fn inference(images: &[f32], weights: &[f32]) -> f32 {
  let loss = do_some_math(images, weights);
  loss
}
```

Happy to accept better names than `DuplicatedNoNeed`. Either way, now we have:

```rust
fn inference(images: &[f32], weights: &[f32]) -> f32 {
  let loss = do_some_math(images, weights);
  loss
}
fn backprop(images: &[f32], weights: &[f32], dweights: &mut [f32]) {
  enzyme_update_inplace_dy(dweights);
}
```

We run backprop to get the gradients to update our weights, tracking of the loss while training is optional. Keep in mind that this will allow Enzyme to do some slightly advanced dead code elimination, but at the end of the day Enzyme will still need to compute most of `do_some_math(x,y)` in order to  calculate `dy`. So how much runtime you save by not asking for loss will depend on your application. We won't introduce a new motivation for our last example, but let's assume we have reasons to only want `dweights`, but do not care about the original weights anymore.

```rust
#[autodiff(backprop, Reverse, Constant, DuplicatedNoNeed, DuplicatedNoNeed)]
fn inference(images: &[f32], weights: &[f32]) -> f32 {
  let loss = do_some_math(images, weights);
  loss
}
```

`DuplicatedNoNeed` allows Enzyme to reuse the memory of our `weigths` variable as a scratchspace. That means it might increase the performance, but in exchange the variable shall not be assumed to have meaningful values afterwards. That's obviously only valid in Julia, C++, etc., but not in Rust. We had some discussion on whether this can be represented as MaybeUninit or Option but didn't got to a conclusion yet. (WIP)

```rust
fn inference(images: &[f32], weights: &[f32]) -> f32 {
  let loss = do_some_math(images, weights);
  loss
}
fn backprop(images: &[f32], weights: &[f32], dweights: &mut [f32]) {
  enzyme_update_inplace_dy(dweights);
}
```

And as the very last one, Enzyme follows Jax and all the other AD tools by allowing batched backpropagation:

```rust
#[autodiff(backprop, Reverse(2), Constant, Duplicated, DuplicatedNoNeed)]
fn inference(images: &[f32], weights: &[f32]) -> f32 {
  let loss = do_some_math(images, weights);
  loss
}
```

We don't expose batchmode on the Rust side yet, let's do one step after the other.

```rust
fn inference(images: &[f32], weights: &[f32]) -> f32 {
  let loss = do_some_math(images, weights);
  loss
}
fn backprop(images: (&[f32], &[f32]), weights: (&[f32], &[f32]), dweights: (&mut f[f32], &mut [f32])) {
  enzyme_update_inplace_dy(dweights.0);
  enzyme_update_inplace_dy(dweights.1);
}
```

Here are actual (compiling) examples:

https://github.com/EnzymeAD/rust/tree/master/library/autodiff/examples


We also ask for a wildcard allowance to recognize ENZYME_ environment variables for debug or profiling purposes. Here are the ones we currently use:

https://github.com/EnzymeAD/rust#enzyme-config

While Enzyme is very fast due to running optimizations before AD, we don't explore all the classical AutoDiff tricks yet. Namely we do miss support for adjusting checkpointing decisions, which describes the question of whether we want to cache or recompute values needed for the gradient computations. It generally lies in NP to find the optimal balance for each given program, but there are good approximations. You can think of it in terms of custom allocators. Replacing the algorithm might affect your runtime performance, but does not affect the result of your function calls. In the future it might be interesting to let the user interact with checkpointing.

Finally, let's assume that you want to use [differentiable rendering](https://arxiv.org/abs/2006.12057), but someone added a "fast" version of the [inverse square root function](https://en.wikipedia.org/wiki/Fast_inverse_square_root#Overview_of_the_code) to your render engine, breaking your AutoDiff tool, which can't figure out how `i  = 0x5f3759df - ( i >> 1 );` would affect your gradient. AutoDiff packages for this reason allow declaring a custom derivative `f'` for a function `f`. In such a case the AD tool will not look at the implementation of `f` and directly use the user provided `f'`. Jax documentation also has a large list of other reasons due to which you might want to use custom derivatives: https://jax.readthedocs.io/en/latest/notebooks/Custom_derivative_rules_for_Python_code.html
Julia has a whole ecosystem called [ChainRules.jl](https://juliadiff.org/ChainRulesCore.jl/stable/) around custom derivatives. Enzyme does support custom derivatives, but we do not expose this feature on the Rust side yet.

## History and status

Enzyme started as a PhD project of William Moses and Valentin Churavy, that was able to differentiate the LLVM-IR generated by a subset of C and Julia. (...)

### Enzyme frontends

Enzyme currently has experimental frontends for C, C++, Julia, Fortran, Numba, and Rust.
We hope that as part of the nightly releases Rust-Enzyme can mature relatively fast because:

1) Unlike Enzyme.jl, Rust won't encounter bugs based on Garbage Collection, JIT, or Type Unstable code.
2) Unlike Clang, we do ship the source code for the standard library. On the Rust side, we therefore don't need to manually add support for functions like [`_ZSt18_Rb_tree_decrementPKSt18_Rb_tree_node_base`](https://github.com/EnzymeAD/Enzyme/pull/764/files#diff-33703e707eb3c80e460e135bec72264fd2380201070a2959c6755bb26c72a504R190).
3) Minimizing Rust code is reasonably nice and cargo makes it easy to reproduce bugs.

### Current limitations

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

## Non-alternatives

TODO: Talk about why this can't be done reasonably in any other way than adding it to the language.

##
