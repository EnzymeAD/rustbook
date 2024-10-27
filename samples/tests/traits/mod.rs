samples::test! {
    volumetric;
    /// ANCHOR: volumetric
    use std::autodiff::autodiff;
    trait Volumetric {
        /// Strain energy density
        fn psi(&self, j: f64) -> f64;
        /// Derivative of strain energy with respect to $J$
        fn d_psi(&self, j: f64, b_psi: f64) -> (f64, f64);
        /// The volumetric contribution to the second Piola-Kirchhoff stress is
        /// a scalar multiplied by $C^{-1}$ where $C = I + 2E$ in terms of the
        /// Green-Lagrange strain $E$. The derivative of $J$ with respect to $E$
        /// is $J C^{-1}$. We'll call the volumetric contribution that scalar
        /// multiple of $C^{-1}$.
        fn stress(&self, j: f64) -> f64 {
            let (_, d_psi) = self.d_psi(j, 1.0);
            d_psi * j
        }
    }

    struct Ogden {
        k: f64,
    }
    impl Ogden {
        pub fn stress_analytic(&self, j: f64) -> f64 {
            self.k * 0.5 * (j * j - 1.0)
        }
    }
    impl Volumetric for Ogden {
        #[autodiff(d_psi, Reverse, Const, Active, Active)]
        fn psi(&self, j: f64) -> f64 {
            self.k * 0.25 * (j * j - 1.0 - 2.0 * j.ln())
        }
    }

    fn main() {
        let j = 0.8;
        let vol = Ogden { k: 1.0 };
        let s = vol.stress(j);
        let s_ref = vol.stress_analytic(j);
        assert!((s - s_ref).abs() < 1e-15, "{}", s - s_ref);
    }
    // ANCHOR_END: volumetric
}
