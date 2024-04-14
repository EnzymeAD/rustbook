samples::test! {
    forward_of_reverse;
    // ANCHOR: forward_of_reverse
    #[autodiff(df, Reverse, Duplicated, Active)]
    fn f(x: &[f32; 2]) -> f32 {
        x[0] * x[0] + x[1] * x[0]
    }

    #[autodiff(dg, Forward, Dual, Dual)]
    fn g(x: &[f32; 2], gx: &mut [f32; 2]) {
        let mut bx = [0.0, 0.0];
        df(x, &mut bx, 1.0);
        *gx = bx;
    }

    fn main() {
        let x = [2.0, 3.0];
        let fx = f(&x);
        assert_eq!(10.0, fx);

        let mut gx = [0.0, 0.0];
        g(&x, &mut gx);
        assert_eq!([7.0, 2.0], gx);

        let dx = [1.0, 0.0];
        let mut dgx = [0.0, 0.0];
        dg(&x, &dx, &mut gx, &mut dgx);
    }
    // ANCHOR_END: forward_of_reverse
}
