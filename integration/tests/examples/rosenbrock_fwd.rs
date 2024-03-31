#![feature(autodiff)]

#[autodiff(d_rosenbrock, Forward, Dual, Dual)]
fn rosenbrock(x: &[f64; 2]) -> f64 {
    let mut res = 0.0;
    for i in 0..(x.len() - 1) {
        let a = x[i + 1] - x[i] * x[i];
        let b = x[i] - 1.0;
        res += 100.0 * a * a + b * b;
    }
    res
}

fn main() {
    let x = [3.14, 2.4];
    let output = rosenbrock(&x);
    println!("{output}");
    let df_dx = d_rosenbrock(&x, &[1.0, 0.0]);
    let df_dy = d_rosenbrock(&x, &[0.0, 1.0]);

    dbg!(&df_dx, &df_dy);
    let dx = df_dx.1;
    let dy = df_dy.1;

    // https://www.wolframalpha.com/input?i2d=true&i=x%3D3.14%3B+y%3D2.4%3B+D%5Brosenbrock+function%5C%2840%29x%5C%2844%29+y%5C%2841%29+%2Cy%5D
    assert!((dx - 9373.54).abs() < 0.1);
    assert!((dy - (-1491.92)).abs() < 0.1);
}

#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        super::main()
    }
}
