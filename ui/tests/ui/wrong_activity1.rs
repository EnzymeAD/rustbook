#![feature(autodiff)]

#[autodiff(d_f1, Reverse, Active, Active)]
fn f1(x: &f64) -> f64 {
    x * x
}
