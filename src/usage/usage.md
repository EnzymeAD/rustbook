# Usage

Enzyme differentiates arbitrary multivariate vector functions as the most general case in automatic differentiation

\\[
f: \mathbb{R}^n \rightarrow \mathbb{R}^m, y = f(x)
\\]

For simplicity we define a vector function with \\(m=1\\). 
However, this tutorial can easily be applied to arbitrary \\(m \in \mathbb{N}\\).

```rust
fn f(x: &[f32], y: &mut f32) {
    *y = x[0] * x[0] + x[1] * x[0];
}
```
We also support functions that return a float value:
```rust
fn g(x: &[f32]) -> f32 {
    x[0] * x[0] + x[1] * x[0]
}
```

## Forward Mode
The forward model is defined as
\\[
\begin{aligned}
y &= f(x) \\\\
\dot{y} &= \nabla f(x) \cdot \dot{x}
\end{aligned}
\\]

To obtain the first element of the gradient using the forward model 
we have to seed \\(\dot{x}\\) with \\(\dot{x}=[1.0,0.0]\\).

In the forward mode the second element which gets added for Dual arguments stores the tangent.
```rust
{{#include ../../samples/tests/forward/mod.rs:empty_return}}
```
In the returning case we would write similar code, note that in this case
the second Dual refers to our return value.
```rust
{{#include ../../samples/tests/forward/mod.rs:dual_return}}
```
Note that to acquire the full gradient one needs to execute the forward model a second time with the seed `dx` set to `[0.0, 1.0]`.


## Reverse Mode

The reverse model in AD is defined as
\\[
\begin{aligned}
y &= f(x) \\\\
\bar{x} &= \bar{y} \cdot \nabla f(x)
\end{aligned}
\\]
bar denotes an adjoint variable. Note that executing AD in reverse mode
computes both ``y`` and the adjoint \\(\bar{x}\\).

Enzyme stores the value and adjoint of a variable when marking a type 
as `Duplicated`. Then the first element represent the value and the second 
the adjoint. Evaluating the reverse model using Enzyme is done in the 
following example.
```rust
{{#include ../../samples/tests/reverse/mod.rs:empty_return}}
```
This yields the gradient of `f` in `bx` at point `x = [2.0, 2.0]`. 
`by` is called the seed and has to be set to ``1.0`` in order to compute 
the gradient. Please note that unlike `Dual`, for `Duplicated` the seed
is getting zeroed, which is required for correctness in certain cases.

We can again also handle functions returning a scalar. In this case we mark the
return value as duplicated. The seed is then going to be an extra,
last input argument.

```rust,ignore
#[autodiff(dg, Reverse, Duplicated, Active)]
fn g(x: &[f32]) -> f32 { ... }

fn main() {
    let x  = [2.0, 2.0];
    let bx = [0.0, 0.0];
    let seed = 1.0;
    let y = dg(&x, &mut dx, seed);
    assert!(dy[0] == 6.0 && dy[1] == 2.0);
}
```

We can now verify that indeed the reverse mode and forward mode yield the same result. 

```rust,ignore
#[autodiff(df_f, Forward, Dual, Dual)]
#[autodiff(df_r, Reverse, Duplicated, Duplicated)]
fn f(x: &[f32], y: &mut f32) { ... }

fn main() {
    let x  = [2.0, 2.0];
    let dx_1 = [1.0, 0.0];
    let dx_2 = [0.0, 1.0];
    let mut y = 0.0;
    let mut dy_f = [0.0, 0.0];
    df_f(&x, &mut dx_1, &mut y, &mut dy_f[0]);
    df_f(&x, &mut dx_2, &mut y, &mut dy_f[1]);
    

    let bx = [0.0, 0.0];
    let y  = 0.0;
    let mut dy_r = 1.0;
    df_r(&x, &mut bx, &mut y, &mut dy_r);

    assert_approx_eq!(dy_f[0], dy_r[0]);
    assert_approx_eq!(dy_f[1], dy_r[1]);
}
```

As we can see, the number of calls under Forward mode scales with the number of 
input values. Reverse mode scales with the number of output parameters, 
and is therefore preferable if we have less outputs than inputs. A common example 
is the training of neural networks, where we have a single output (loss), 
but a large input (weights).
