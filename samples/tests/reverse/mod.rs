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
    active_only;
    /// ANCHOR:  active_only
    #[autodiff(d_f, Reverse, Active, Active)]
    #[autodiff(d_f2, Reverse, Active, ActiveOnly)]
    fn f(x: f64) -> f64 {
        f64::sin(x)
    }

    fn main() {
        let x = 1.0;
        let (_y, d_y) = d_f(x, 1.0);
        let d2_y = d_f2(x, 1.0);
        let cos_x = f64::cos(x);
        assert!((d2_y - d_y).abs() < 1e-15);
        assert!((cos_x - d_y).abs() < 1e-15);
    }
    /// ANCHOR_END: active_only
}

samples::test! {
    self_duplicated;
    struct Ogden {
        k: f64,
    }
    impl Ogden {
        #[autodiff(d_f, Reverse, Duplicated, Const, Active)]
        fn f(&self, _j: f64) -> f64 {
            self.k * self.k
        }
    }

    fn main() {
        let j = 4.0;
        let vol = Ogden { k: 1.0 };
        let mut out = Ogden { k: 0.0 };
        let _ = vol.d_f(&mut out, j, 1.0);
        let res = 2.0 * vol.k;
        assert!((out.k - res).abs() < 1e-15, "{} {}", res, out.k);
    }
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
        let mut by = 1.0; // seed
        df(&x, &mut bx, &mut y, &mut by);
        assert_eq!([7.0, 2.0], bx);
        assert_eq!(10.0, y);
        assert_eq!(0.0, by); // seed is zeroed
    }
    // ANCHOR_END: empty_return
}

samples::test! {
    active_return;
    // ANCHOR: active_return
    #[autodiff(df, Reverse, Duplicated, Active)]
    fn f(x: &[f32; 2]) -> f32 {
        x[0] * x[0] + x[1] * x[0]
    }

    fn main() {
        let x = [2.0, 3.0];
        let mut bx = [0.0, 0.0];
        let by = 1.0; // seed
        let y = df(&x, &mut bx, by);
        assert_eq!([7.0, 2.0], bx);
        assert_eq!(10.0, y);
    }
    // ANCHOR_END: active_return
}

#[cfg(broken)]
samples::test! {
    forward_and_reverse;
    // ANCHOR: forward_and_reverse
    #[autodiff(df_fwd, Forward, Dual, Dual)]
    #[autodiff(df_rev, Reverse, Duplicated, Duplicated)]
    fn f(x: &[f32; 2], y: &mut f32) {
        *y = x[0] * x[0] + x[1] * x[0];
    }

    fn main() {
        let x = [2.0, 3.0];

        // Compute gradient via forward-mode
        let dx_0 = [1.0, 0.0];
        let dx_1 = [0.0, 1.0];
        let mut y = 0.0;
        let mut dy_f = [0.0, 0.0];
        df_fwd(&x, &dx_0, &mut y, &mut dy_f[0]);
        df_fwd(&x, &dx_1, &mut y, &mut dy_f[1]);
        assert_eq!([7.0, 2.0], dy_f);

        // Compute gradient via reverse-mode
        let mut bx = [0.0, 0.0];
        let mut y = 0.0;
        let mut by = 1.0; // seed
        df_rev(&x, &mut bx, &mut y, &mut by);
        assert_eq!([7.0, 2.0], bx);
        assert_eq!(10.0, y);
        assert_eq!(0.0, by); // seed is zeroed
    }
    // ANCHOR_END: forward_and_reverse
}

#[cfg(broken)]
samples::test! {
    all_active;
    // ANCHOR: all_active
    #[autodiff(df, Reverse, Active, Active, Active)]
    fn f(x: f32, y: f32) -> f32 {
        x * x + 3.0 * y
    }

    fn main() {
        let (x, y) = (5.0, 7.0);
        let (z, bx, by) = df(x, y, 1.0);
        assert_eq!(46.0, z);
        assert_eq!(10.0, bx);
        assert_eq!(3.0, by);
    }
    // ANCHOR_END: all_active
}
