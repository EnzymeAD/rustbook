#![feature(autodiff)]
use std::autodiff::autodiff;

#[autodiff(d_square, Reverse, Duplicated, Const, Active)]
fn square(x: &f64) -> f64 {
    x * x
}
