samples::test! {
    forward_of_reverse;
    // ANCHOR: forward_of_reverse
    use std::autodiff::autodiff;
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

samples::test! {
    higher;
    use std::autodiff::autodiff;
    // A direct translation of
    // https://enzyme.mit.edu/index.fcgi/julia/stable/generated/autodiff/#Forward-over-reverse

    #[autodiff(ddf, Forward, Dual, Dual, Dual, Dual)]
    fn df2(x: &[f32;2], dx: &mut [f32;2], out: &mut [f32;1], dout: &mut [f32;1]) {
        df(x, dx, out, dout);
    }

    #[autodiff(df, Reverse, Duplicated, Duplicated)]
    fn f(x: &[f32;2], y: &mut [f32;1]) {
        y[0] = x[0] * x[0] + x[1] * x[0]
    }

    fn main() {
        let mut y = [0.0];
        let x = [2.0, 2.0];

        let mut dy = [0.0];
        let mut dx = [1.0, 0.0];

        let mut bx = [0.0, 0.0];
        let mut by = [1.0];
        let mut dbx = [0.0, 0.0];
        let mut dby = [0.0];

        ddf(&x, &mut bx, &mut dx, &mut dbx,
            &mut y, &mut by, &mut dy, &mut dby);
    }
}
