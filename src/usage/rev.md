# Reverse Mode
# Autodiff on LLVM IR

TODO: Typical LICM \\(O(n)\\) vs \\(O(n^2)\\) Enzyme example.
TODO: Talk about what makes this approach special and a good fit for Rust conceptually.

## Changes to Rust

TODO: Talk about the new attributes and define the semantics of these new attributes.  Give examples.



#### Reverse Mode
Both the inplace and "normal" variant return the gradient. The difference is that with `Active` the gradient is returned and with `Duplicated` the gradient is accumulated in-place.




### Usage story
Let us start by looking at the most basic examples we can think of:

\\[ f(x,y) = x^2 + 3y \\]

We have two input variables \\(x\\), \\(y\\) and a scalar return value.
The gradient is

\\[ \nabla f = \Big[\frac{\partial f}{\partial x}, \frac{\partial f}{\partial y} \Big] = \big[2x, 3 \big] \\]

Let's check for Enzyme (our compiler explorer does not handle Rust yet, so you'll have to trust me on this).

```rust,noplayground
{{#include ../../samples/tests/reverse/mod.rs:all_active}}
```

Enzyme actually generates the code on LLVM-IR level, but Rust is nicer to read, so I will pretend we would generate a Rust implementation:

```rust,ignore
fn f(x: f32, y: f32) -> f32 {
  x * x + 3.0 * y
}
fn df(x: f32, y: f32) -> (f32, f32, f32) {
  let d_dx = 2.0 * x;
  let d_dy = 3.0;
  let f = x * x + 3.0 * y;
  (d_dx, d_dy, f)
}
```

Note that the last entry in the result tuple contains the original return value. However, we don't always pass things by value, so let's make sure we have a sensible solution:

```rust,ignore
#[autodiff(df, Reverse, Active, Duplicated, Active)]
fn f(x: f32, y: &f32) -> f32 {
  x * x + 3.0 * y
}
```

(pay attention to `y`).

```rust,ignore
fn f(x: f32, y: f32) -> f32 {
  x * x + 3.0 * y
}
fn df(x: f32, y: &f32, d_dy: &mut f32) -> (f32, f32) {
  let d_dx = 2.0 * x;
  *d_dy += 3.0;
  let f = x * x + 3.0 * y
  (d_dx, f)
}
```

In the case of references (or pointers) we do expect the user to create `d_dy`.

We could obviously zero-initialize a float for the user, but let's assume the constructor is complex due to involving a double-linked-list or ffi, so we can't guarantee that on the compiler side. As an alternative motivation, imagine that we call `df` 5 times in a loop. It is clear that in this case the accumulated gradients should be 5 times higher too, which won't happen if we set `d_dy = 3.0` each time, instead of using `+=`. Yet another reason would be higher-order derivatives (todo: just refer to literature?).

Now that we got back from this rabbit hole, let's go wild and train a neural network on our local national lab server:

```rust,ignore
#[autodiff(backprop, Reverse, Duplicated, Duplicated, Active)]
fn training_loss(images: &[f32], weights: &[f32]) -> f32 {
  let loss = do_some_math(images, weights);
  loss
}
```

Now Enzyme gives us:

```rust,ignore
fn training_loss(images: &[f32], weights: &[f32]) -> f32 {
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

```rust,ignore
#[autodiff(backprop, Reverse, Constant, Duplicated, Active)]
fn training_loss(images: &[f32], weights: &[f32]) -> f32 {
  let loss = do_some_math(images, weights);
  loss
}
```

After all, we shouldn't modify our train and test images to improve our accuracy, right? So we now generate:

```rust,ignore
fn training_loss(images: &[f32], weights: &[f32]) -> f32 {
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

```rust,ignore
#[autodiff(backprop, Reverse, Constant, Duplicated, DuplicatedNoNeed)]
fn training_loss(images: &[f32], weights: &[f32]) -> f32 {
  let loss = do_some_math(images, weights);
  loss
}
```

Happy to accept better names than `DuplicatedNoNeed`. Either way, now we have:

```rust,ignore
fn training_loss(images: &[f32], weights: &[f32]) -> f32 {
  let loss = do_some_math(images, weights);
  loss
}
fn backprop(images: &[f32], weights: &[f32], dweights: &mut [f32]) {
  enzyme_update_inplace_dy(dweights);
}
```

We run backprop to get the gradients to update our weights, tracking of the loss while training is optional. Keep in mind that this will allow Enzyme to do some slightly advanced dead code elimination, but at the end of the day Enzyme will still need to compute most of `do_some_math(x, y)` in order to  calculate `dy`. So how much runtime you save by not asking for loss will depend on your application. We won't introduce a new motivation for our last example, but let's assume we have reasons to only want `dweights`, but do not care about the original weights anymore.

```rust,ignore
#[autodiff(backprop, Reverse, Constant, DuplicatedNoNeed, DuplicatedNoNeed)]
fn training_loss(images: &[f32], weights: &[f32]) -> f32 {
  let loss = do_some_math(images, weights);
  loss
}
```

`DuplicatedNoNeed` allows Enzyme to reuse the memory of our `weigths` variable as a scratchspace. That means it might increase the performance, but in exchange the variable shall not be assumed to have meaningful values afterwards. That's obviously only valid in Julia, C++, etc., but not in Rust. We had some discussion on whether this can be represented as MaybeUninit or Option but didn't got to a conclusion yet. (WIP)

```rust,ignore
fn training_loss(images: &[f32], weights: &[f32]) -> f32 {
  let loss = do_some_math(images, weights);
  loss
}
fn backprop(images: &[f32], weights: &[f32], dweights: &mut [f32]) {
  enzyme_update_inplace_dy(dweights);
}
```

And as the very last one, Enzyme follows Jax and all the other AD tools by allowing batched backpropagation:

```rust,ignore
#[autodiff(backprop, Reverse(2), Constant, Duplicated, DuplicatedNoNeed)]
fn training_loss(images: &[f32], weights: &[f32]) -> f32 {
  let loss = do_some_math(images, weights);
  loss
}
```

We don't expose batchmode on the Rust side yet, let's do one step after the other.

```rust,ignore
fn training_loss(images: &[f32], weights: &[f32]) -> f32 {
  let loss = do_some_math(images, weights);
  loss
}
fn backprop(images: (&[f32], &[f32]), weights: (&[f32], &[f32]), dweights: (&mut f[f32], &mut [f32])) {
  enzyme_update_inplace_dy(dweights.0);
  enzyme_update_inplace_dy(dweights.1);
}
```

