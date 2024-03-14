samples::test! {
    square;
    /// ANCHOR: square
    #[autodiff(d_square, Reverse, Duplicated, Active)]
    fn square(x: &f64) -> f64 {
        x * x
    }

    fn main() {
        let x = 3.0;
        let output = square(&x);
        assert_eq!(9.0, output);

        let mut df_dx = 0.0;
        let output_ = d_square(&x, &mut df_dx, 1.0);
        assert_eq!(output, output_);
        assert_eq!(6.0, df_dx);
    }
    // ANCHOR_END: square
}

samples::test! {
    empty_return;
    // ANCHOR: empty_return
    #[autodiff(df, Reverse, Duplicated, Duplicated)]
    fn f(x: &[f32; 2], y: &mut f32) {
        *y = x[0] * x[0] + x[1] * x[0];
    }

    fn main() {
        let x = [2.0, 3.0];
        let mut bx = [0.0, 0.0];
        let mut y = 0.0;
        let mut by = 1.0;
        df(&x, &mut bx, &mut y, &mut by);
        assert_eq!([7.0, 2.0], bx);
        assert_eq!(10.0, y);
        // by can be overwritten so its value is unspecified
    }
    // ANCHOR_END: empty_return
}
