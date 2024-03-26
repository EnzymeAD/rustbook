# Design:
There are naturally three locations where we could introduce our autodiff macro: on the definition side of f, on the definition side of df, or on the call-site of df.  
We only have implemented support for the first two, because this is enough to implement the third one with a normal rust macro. Let us look at some example:

```rust
#![feature(autodiff)]
#[autodiff(df, Reverse, Active, Active)]
fn fsquare(x: f32) -> f32 {
  x * x
}

fn main() {
  let x = 3.14;
  let res = fsquare(x);
  let (res_, dres) = df(x, 1.0);
  // let dres = autodiff!(f, x);
  assert_eq!(dres, 2.0 * x);
  assert_eq!(res, res_);
}
```

Some tools always compute all gradients, or rely on Dead-Code-Elimination to remove code that would compute gradients unused by users. In comparison, Enzyme does only generate the code needed to compute your gradients from the beginning (and uses and extends various LLVM passes to do so). Therefore it is crucial that users (can) specify only the minimal set of variables as active. Library authors (e.g. faer, minmax, ...) can not reliably predict which gradients users will need, and offerint all would cause a combinatorical explosion. We therefore allow users to differentiate functions specified in a dependency. While not technically required, we do enforce that this function in the dependency is marked as public, to not violate Rusts visibility behaviour. We believe for now that this is the best compromise for usability, but would be happy to hear feedback. An example:

```rust,compile_fail
# #![feature(autodiff)]
// ; dependency.rs
fn f(x: f32) -> f32 {
  x * x
}

// ; main.rs
#[autodiff(f, Reverse, Active, Active)]
fn df(x: f32, d_df: f32) -> (f32, f32);

fn main() {
  let x = 3.14;
  let res = f(x);
  let (_, dres) = df(x, 1.0);
  assert_eq!(dres, 2.0 * x);
}
```
The `;` is our way of ensuring that users can't provide an implementation to it that would later get overwritten by the autodiff tool. We desugare it into a combination of `unimplemented!()`, inline-asm (noop), and bench-blackbox, to avoid the method being inlined. We take some additional care of this within the compiler.  


Let's look at the configuration now:
1) The first argument of our macro is the name (or path) to the "other" function. That is, if we apply our macro to `f`, then we would put the name of `df`, which the macro will then generate. If we put our macro on top of an empty function, then the name should be the one of the function which we want to differentiate.
2) Forward vs Reverse mode: From a math point of view, this one describes in which order we will expand the chain rule, or rather if we want to calculate the vector-jacobian product, or the jacobian-vector product. It has a very large impact on the performance and the following config parameters will differ slightly based on the mode. No real impact beside of performance. Technically something in between Forward and Reverse would be possible, but has not been explored much by Jax, Enzyme, or others.
3) Activity of arguments, in the order in which they appear in `f`. If we use ReverseMode and return a non `()` value, we specify activity for the return too. We are open to discuss the actual names here:`Const`, `Active`, `Duplicated`, `DuplicatedNoNeed`.
