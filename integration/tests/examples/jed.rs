#![feature(autodiff)]

#[autodiff(d_square, Reverse, Duplicated, Active)]
fn square(x: &f64) -> f64 {
    x.powi(2)
}

fn main() {
    let x = 3.0;
    let output = square(&x);
    println!("{output}");

    let mut df_dx = 0.0;
    d_square(&x, &mut df_dx, 1.0);
    println!("df_dx: {:?}", df_dx);
}

