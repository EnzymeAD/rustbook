samples::test! {
    empty_return;
    // ANCHOR: empty_return
    use std::autodiff::autodiff;
    #[autodiff(df, Forward, Dual, Dual)]
    fn f(x: &[f32; 2], y: &mut f32) {
        *y = x[0] * x[0] + x[1] * x[0];
    }

    fn main() {
        let x = [2.0, 3.0];
        let dx = [1.0, 0.0];
        let mut y = 0.0;
        let mut dy = 0.0;
        df(&x, &dx, &mut y, &mut dy);
        assert_eq!(10.0, y);
        assert_eq!(7.0, dy);
    }
    // ANCHOR_END: empty_return
}

samples::test! {
    dual_return;
    // ANCHOR: dual_return
    use std::autodiff::autodiff;
    #[autodiff(df, Forward, Dual, Dual)]
    fn f(x: &[f32; 2]) -> f32 { x[0] * x[0] + x[1] * x[0] }

    fn main() {
        let x  = [2.0, 2.0];
        let dx = [1.0, 0.0];
        let (y, dy) = df(&x, &dx);
        assert_eq!(dy, 2.0 * x[0] + x[1]);
        assert_eq!(y, f(&x));
    }
    // ANCHOR_END: dual_return
}

samples::test! {
    dual_only_return;
    // ANCHOR: dual_only_return
    use std::autodiff::autodiff;
    #[autodiff(df, Forward, Dual, Dual)]
    #[autodiff(df2, Forward, Dual, DualOnly)]
    fn f(x: &[f32; 2]) -> f32 { x[0] * x[0] + x[1] * x[0] }

    fn main() {
        let x  = [2.0, 2.0];
        let dx = [1.0, 0.0];
        let (y, dy) = df(&x, &dx);
        let dy2 = df2(&x, &dx);
        assert_eq!(dy, 2.0 * x[0] + x[1]);
        assert_eq!(dy2, dy);
        assert_eq!(y, f(&x));
    }
    // ANCHOR_END: dual_only_return
}
