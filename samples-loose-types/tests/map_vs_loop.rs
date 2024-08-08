#![feature(autodiff)]

#[autodiff(sin_vec, Reverse, Duplicated, Active)]
fn cos_vec(x: &Vec<f32>) -> f32 {

    let theta_inverse = 1.0 / x.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
    let mut w = [0.; 3];
    for i in 0..3 {
        w[i] = x[i] * theta_inverse;
    }
    w.iter().sum()
}

#[autodiff(sin_vec2, Reverse, Duplicated, Active)]
fn cos_vec2(x: &Vec<f32>) -> f32 {

    let theta_inverse = 1.0 / x.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
    let w = x.iter().map(|v| v * theta_inverse);
    w.sum()
}

#[test]
fn compare() {
    let x = vec![1.0, 2.0, 3.0];
    let x2 = vec![1.0, 2.0, 3.0];
    let mut d_x = vec![0.0; 3];
    let mut d_x2 = vec![0.0; 3];

    sin_vec(&x, &mut d_x, 1.0);
    sin_vec2(&x, &mut d_x, 1.0);

    dbg!(&d_x, &x);
    dbg!(&d_x2, &x2);

    for i in 0..3 {
        assert!((d_x[i] - d_x2[i]).abs() < 1e-6);
    }
}
