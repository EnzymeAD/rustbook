#![feature(autodiff)]

#[autodiff(df, Forward, Dual, Dual)]
fn f(x: &[f32]) -> f32 { x[0] * x[0] + x[1] * x[0] }

fn main() {
    let x  = [2.0, 2.0];
    let dx = [1.0, 0.0];
    let (y, dy) = df(&x, &dx);
    assert_eq!(dy, 2.0 * x[0] + x[1]);
}