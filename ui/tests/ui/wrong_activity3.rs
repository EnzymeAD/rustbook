#![feature(autodiff)]

#[autodiff(d_f3, Reverse, Active, Active)]
fn f3(x: i64) -> f64 {
    (x * x) as f64
}
