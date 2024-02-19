# Unsafe Interface

Especially Reverse Mode AD has various code transformations which can easily 
cause UB, when not used correctly. We work on catching all cases through our 
design and additional safety checks, but are aware that for some cases like
hot loops, or GPU Kernels such checks are not desired.

A motivational (C++) example is given [here](https://enzyme.mit.edu/getting_started/CallingConvention/#result-only-duplicated-argument)
This example would be instant UB in Rust because Types must never be in an invalid
state, which is not the case for loss here.
In our safe interface we solve this by accepting Types by value, making the invalid state inaccessible from Rust code. This however requires allocating a new variable for each call, which could become a performance limitation.

While not implemented yet, we propose a second interface, with the bikeshedding 
name `unsafe_ad` instead of `autodiff`. It will `not` generate the safety checks 
which we discussed in previous sections. The generated Interface also differs for 
`DualOnly` and `DuplicatedOnly`. See this examples:

```rust
fn f(x: &[f32], y: &mut f32) {
    y = x[0] * x[0] + x[1] * x[0];
}

#[autodiff(df, Forward, Dual, Dual)]
fn f(x: &[f32], y: &mut f32) { ... }
// fn df(x: &[f32], dx: &mut [f32], y: &mut f32, dy: &mut f32);

fn main() {
    let x  = [2.0, 2.0];
    let dx = [1.0, 0.0];
    let y  = 0.0
    let dy = 0.0;
    df(&x, &mut dx, &mut y, &mut dy);
}
```

The first optimization here is `DualOnly`. Now 
```rust
#[autodiff(df, Forward, Dual, Dual)]
fn f(x: &[f32], y: &mut f32) { ... }
// fn df(x: Vec<f32>, dx: &mut[f32], y: f32, dy: &mut f32);
```
Both x and y become inaccessible, so no harm can happen. But let us assume
that we have to reuse these expensive memory allocations.

```rust 
#[unsafe_ad(df, Forward, Dual, Dual)]
fn f(x: &[f32], y: &mut f32) { ... }
// unsafe fn df(x: MaybeUninit<[f32]>, dx: &mut[f32], y: MaybeUninit<&f32>, dy: &mut f32);
```
We expect both x and y to be in a valid state, however they will be in an invalid state after calling df. This allows us to omit computing their original output value and it also allows us to re-use it's memory location as buffer memory.







