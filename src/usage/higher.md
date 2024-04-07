# Higher Order Derivatives

Computing higher order derivatives like hessians can be done with Enzyme by differentiating functions that compute lower order derivatives. 
This requires that functions are differentiated in the right order, which we currently don't handle.
As a workaround, we introduce two new AD modes `ForwardFirst` and ReverseFirst` that will be differentiated (and optimized)
before we differentiate the default `Forward` and `Reverse` mode invocations. An example is given below.


```rust
// A direct translation of
// https://enzyme.mit.edu/index.fcgi/julia/stable/generated/autodiff/#Forward-over-reverse

#[autodiff(ddf, Forward, Dual, Dual, Dual, Dual)]
fn df2(x: &[f32;2], dx: &mut [f32;2], out: &mut [f32;1], dout: &mut [f32;1]) {
    df(x, dx, out, dout);
}

#[autodiff(df, ReverseFirst, Duplicated, Duplicated)]
fn f(x: &[f32;2], y: &mut [f32;1]) {
    y[0] = x[0] * x[0] + x[1] * x[0]
}

#[test]
fn main() {
    let mut y = [0.0];
    let x = [2.0, 2.0];

    let mut dy = [0.0];
    let mut dx = [1.0, 0.0];

    let mut bx = [0.0, 0.0];
    let mut by = [1.0];
    let mut dbx = [0.0, 0.0];
    let mut dby = [0.0];

    ddf(&x, &mut bx, &mut dx, &mut dbx, 
        &mut y, &mut by, &mut dy, &mut dby);
}
```
