#![feature(autodiff)]

#[autodiff(d_f2, Reverse, Duplicated, Active)]
fn f2(x: f64) -> f64 {
    x * x
}
