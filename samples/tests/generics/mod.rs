samples::test! {
    generic;
    // ANCHOR: square
    #[autodiff(d_square, Reverse, Duplicated, Active)]
    fn square<T: std::ops::Mul<Output = T> + Copy>(x: &T) -> T {
        *x * *x
    }

    fn main() {
        let xf32: f32 = 3.0;
        let xf64: f64 = 3.0;
        let outputf32 = square::<f32>(&xf32);
        let outputf64 = square::<f64>(&xf64);
        assert_eq!(9.0, outputf32);
        assert_eq!(9.0, outputf64);

        let mut df_dxf32: f32 = 0.0;
        let mut df_dxf64: f64 = 0.0;
        let output_f32 = d_square::<f32>(&xf32, &mut df_dxf32, 1.0);
        let output_f64 = d_square::<f64>(&xf64, &mut df_dxf64, 1.0);
        assert_eq!(outputf32, output_f32);
        assert_eq!(outputf64, output_f64);
        assert_eq!(6.0, df_dxf32);
        assert_eq!(6.0, df_dxf64);
    }
    // ANCHOR_END: generic
}

