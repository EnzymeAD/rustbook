# Higher Order Derivatives

Computing higher order derivatives like hessians can be done with Enzyme by differentiating functions that compute lower order derivatives. 
[This example](https://github.com/EnzymeAD/rust/blob/master/library/autodiff/examples/hessian_sin.rs) requires that rustc first uses Enzyme to fill the implementation of the `jac` function, before it uses Enzyme to fill the implementation of `hess`, by differentiatng `jac`. 
This is currently not guaranteed and only works by coincidence in some cases. 
This should be easy to fix, so please reach out if you would like to contribute and need some help to get started!

Enzyme also considers adding helper function to directly compute common higher order derivatives in the future.

