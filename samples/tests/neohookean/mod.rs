#![allow(non_snake_case)]
use std::autodiff::autodiff;

use std::ops::{Add, Mul, Sub};

type Mat3x3 = [[f64; 3]; 3];

fn matmul(a: &Mat3x3, at: bool, b: &Mat3x3, bt: bool) -> Mat3x3 {
    let mut c = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                let aa = if at { a[k][i] } else { a[i][k] };
                let bb = if bt { b[j][k] } else { b[k][j] };
                c[i][j] += aa * bb;
            }
        }
    }
    c
}

fn matinv(a: &Mat3x3) -> Mat3x3 {
    let mut b = [
        [
            a[1][1] * a[2][2] - a[1][2] * a[2][1],
            a[1][2] * a[2][0] - a[1][0] * a[2][2],
            a[1][0] * a[2][1] - a[1][1] * a[2][0],
        ],
        [
            a[0][2] * a[2][1] - a[0][1] * a[2][2],
            a[0][0] * a[2][2] - a[0][2] * a[2][0],
            a[0][1] * a[2][0] - a[0][0] * a[2][1],
        ],
        [
            a[0][1] * a[1][2] - a[0][2] * a[1][1],
            a[0][2] * a[1][0] - a[0][0] * a[1][2],
            a[0][0] * a[1][1] - a[0][1] * a[1][0],
        ],
    ];
    let det = a[0][0] * b[0][0] + a[0][1] * b[0][1] + a[0][2] * b[0][2];
    for i in 0..3 {
        for j in 0..3 {
            b[i][j] /= det;
        }
    }
    b
}

/// Kelvin-Mandel 6-vector representation of symmetric 3x3 matrices.
///
/// [Kelvin-Mandel](https://en.wikipedia.org/wiki/Voigt_notation#Mandel_notation)
/// notation has the some ordering as Voigt, but a normalization such that the
/// Frobenius matrix inner product is equivalent to the standard inner product
/// on KM vectors. This is useful for geometry in return mapping algorithms for
/// plasticity.
#[derive(Debug, Clone, Copy)]
pub struct KM {
    pub vals: [f64; 6],
}

impl KM {
    pub fn from_voigt(v: &[f64; 6]) -> Self {
        let d = 2.0_f64.sqrt();
        Self {
            vals: [v[0], v[1], v[2], d * v[3], d * v[4], d * v[5]],
        }
    }

    pub fn zero() -> Self {
        Self { vals: [0.0; 6] }
    }

    pub fn identity() -> Self {
        Self {
            vals: [1.0, 1.0, 1.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn scale(&self, scale: f64) -> Self {
        Self {
            vals: self.vals.map(|v| v * scale),
        }
    }

    pub fn from_binary_op(x: KM, y: KM, op: impl Fn(f64, f64) -> f64) -> Self {
        let mut result = KM::zero();
        result
            .vals
            .iter_mut()
            .zip(x.vals.into_iter().zip(y.vals.into_iter()))
            .for_each(|(r, (x, y))| *r = op(x, y));
        result
    }

    pub fn from_matrix(a: Mat3x3) -> Self {
        let s = 0.5_f64.sqrt();
        KM {
            vals: [
                a[0][0],
                a[1][1],
                a[2][2],
                s * (a[1][2] + a[2][1]),
                s * (a[0][2] + a[2][0]),
                s * (a[0][1] + a[1][0]),
            ],
        }
    }

    pub fn to_matrix(&self) -> Mat3x3 {
        let d = 0.5_f64.sqrt();
        let v = &self.vals;
        [
            [v[0], d * v[5], d * v[4]],
            [d * v[5], v[1], d * v[3]],
            [d * v[4], d * v[3], v[2]],
        ]
    }

    // Stably evaluate e = (F F^T - I)/2 from displacement gradient H = F - I.
    pub fn green_euler(H: Mat3x3) -> Self {
        Self::from_matrix(H) + 0.5 * Self::from_matrix(matmul(&H, false, &H, true))
    }

    // infinitesimal strain using current configuration displacement gradient ddu/dx = ddu/dX F^{-1}
    pub fn epsilon(ddudx: &Mat3x3) -> Self {
        Self::from_matrix(*ddudx)
    }

    pub fn trace(&self) -> f64 {
        let v = &self.vals;
        v[0] + v[1] + v[2]
    }

    pub fn det(&self) -> f64 {
        let m = self.to_matrix();
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    }

    // Compute b = I + 2e or C = I + 2E.
    pub fn cauchy_green(&self) -> Self {
        let v = &self.vals;
        Self {
            vals: [
                1.0 + 2.0 * v[0],
                1.0 + 2.0 * v[1],
                1.0 + 2.0 * v[2],
                2.0 * v[3],
                2.0 * v[4],
                2.0 * v[5],
            ],
        }
    }

    pub fn inv(&self) -> Self {
        let a = self.to_matrix();
        KM::from_matrix(matinv(&a))
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.vals
            .into_iter()
            .zip(other.vals)
            .map(|(a, b)| a * b)
            .sum()
    }

    pub fn norm(&self) -> f64 {
        self.dot(self).sqrt()
    }
}

impl Add for KM {
    type Output = KM;
    fn add(self, rhs: Self) -> Self::Output {
        KM::from_binary_op(self, rhs, f64::add)
    }
}

impl Mul for KM {
    type Output = KM;
    fn mul(self, rhs: Self) -> Self::Output {
        let a = self.to_matrix();
        let b = rhs.to_matrix();
        let c = matmul(&a, false, &b, false);
        KM::from_matrix(c)
    }
}

impl Mul<&KM> for f64 {
    type Output = KM;
    fn mul(self, rhs: &KM) -> Self::Output {
        rhs.scale(self)
    }
}

impl Mul<KM> for f64 {
    type Output = KM;
    fn mul(self, rhs: KM) -> Self::Output {
        rhs.scale(self)
    }
}

impl Sub for KM {
    type Output = KM;
    fn sub(self, rhs: Self) -> Self::Output {
        KM::from_binary_op(self, rhs, f64::sub)
    }
}

pub struct NH {
    pub mu: f64,
    pub lambda: f64,
}

impl NH {
    pub fn from_lame(lambda: f64, mu: f64) -> Self {
        Self { lambda, mu }
    }
    // https://en.wikipedia.org/wiki/Lam%C3%A9_parameters
    pub fn from_youngs(E: f64, nu: f64) -> Self {
        Self::from_lame(
            E * nu / ((1.0 + nu) * (1.0 - 2.0 * nu)),
            E / (2.0 * (1.0 + nu)),
        )
    }

    pub fn stress(&self, e: &KM) -> KM {
        let mut tau = KM::zero();
        stress_enz(e, self, &mut tau);
        tau
    }

    #[allow(dead_code)]
    pub fn d_stress(&self, e: &KM, de: &KM) -> KM {
        let mut tau = KM::zero();
        let mut dtau = KM::zero();
        d_stress_enz(e, de, self, &mut tau, &mut dtau);
        dtau
    }
}

// We can only differentiate free functions, not methods (yet)
// Helmholtz free energy density
#[autodiff(d_psi, Reverse, Duplicated, Const, Active)]
fn psi(e: &KM, nh: &NH) -> f64 {
    let mu = nh.mu;
    let lambda = nh.lambda;
    let J = e.cauchy_green().det().sqrt();
    0.25 * lambda * (J * J - 1.0 - 2.0 * J.ln()) + mu * (e.trace() - J.ln())
}

#[autodiff(d_stress_enz, Forward, Dual, Const, Dual)]
fn stress_enz(e: &KM, nh: &NH, tau: &mut KM) {
    let mut dpsi_de = KM::zero();
    d_psi(&e, &mut dpsi_de, &nh, 1.0);
    let b = e.cauchy_green();
    *tau = dpsi_de * b;
}

pub mod analytic {
    use super::*;
    pub fn stress(e: &KM, nh: &NH) -> KM {
        let lambda = nh.lambda;
        let mu = nh.mu;
        let b = e.cauchy_green();
        let J = b.det().sqrt();
        let I = KM::identity();
        0.5 * lambda * (J * J - 1.0) * I + 2.0 * mu * e
    }

    #[allow(dead_code)]
    pub fn d_stress(e: &KM, ddudx: &Mat3x3, nh: &NH) -> KM {
        let lambda = nh.lambda;
        let mu = nh.mu;
        let b = e.cauchy_green();
        let J = b.det().sqrt();
        let tau = stress(e, nh).to_matrix();
        let deps = KM::epsilon(ddudx);
        let I = KM::identity();
        let FdSFt = lambda * J * J * deps.trace() * I + (2.0 * mu - lambda * (J * J - 1.0)) * deps;
        2.0 * KM::from_matrix(matmul(&ddudx, false, &tau, false)) + FdSFt
    }
}

fn rel_diff(a: KM, a_ref: KM) -> f64 {
    let num = (a - a_ref).norm();
    let den = a_ref.norm();
    num / den
}

#[test]
fn check_norm() {
    let x = KM::from_voigt(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let reference = (1.0_f64 + 4.0 + 9.0 + 2.0 * (16.0 + 25.0 + 36.0)).sqrt();
    assert_eq!(x.norm(), reference);
}

#[autodiff(d_logdet, Reverse, Duplicated, Active)]
fn logdet(b: &KM) -> f64 {
    b.det().ln()
}

#[test]
fn check_inverse() {
    let b = KM::from_voigt(&[1.0, 2.0, -3.2, 0.4, 0.5, 0.6]);
    // This matrix has negative determinant
    assert!((b.det() + 5.668).abs() < 1e-14);
    // Evaluating logdet(&b) would produce NaN (because determinant is
    // negative) so finite differencing can't solve this problem, but the
    // math for the gradient still works out.
    let mut c = KM::zero();
    d_logdet(&b, &mut c, 1.0);
    // The gradient of logdet is the inverse, so test that.
    let a = c * b - KM::identity();
    assert!(a.norm() < 1e-14);
}

#[test]
fn check_stress_direct() {
    let nh = NH::from_youngs(1.0, 0.3);
    let e = KM::from_voigt(&[0.1, 0.2, -0.27, 0.12, 0.13, 0.14]);
    let mut tau = KM::zero();
    stress_enz(&e, &nh, &mut tau);
    let tau_analytic = analytic::stress(&e, &nh);
    let diff = rel_diff(tau, tau_analytic);
    assert!(diff < 1e-14);
}

#[test]
fn check_stress() {
    let nh = NH::from_youngs(1.0, 0.3);
    let e = KM::from_voigt(&[0.1, 0.2, -0.27, 0.12, 0.13, 0.14]);
    let tau = nh.stress(&e);
    let tau_analytic = analytic::stress(&e, &nh);
    let diff = rel_diff(tau, tau_analytic);
    assert!(diff < 1e-14);
}

#[cfg(broken)] // This case requires ENZYME_LOOSE_TYPES=1; see samples-loose-types
#[test]
fn check_dstress() {
    let nh = NH::from_youngs(1.0, 0.3);
    let e = KM::from_voigt(&[0.1, 0.2, -0.27, 0.12, 0.13, 0.14]);
    let tau = nh.stress(&e);
    let ddudx = [[1.0, 1.1, 1.2], [1.3, 1.4, 1.5], [1.6, 1.7, 1.8]];
    let deps = KM::epsilon(&ddudx);
    let de = 2.0 * KM::from_matrix(matmul(&ddudx, false, &e.to_matrix(), false)) + deps;

    // Use finite differencing to check that analytic is correct
    let h = 1e-8;
    let e_h = e + h * de;
    let mut tau_h = KM::zero();
    stress_enz(&e_h, &nh, &mut tau_h);
    let dtau_h = 1.0 / h * (tau_h - tau);
    let dtau_analytic = analytic::d_stress(&e, &ddudx, &nh);
    let diff = rel_diff(dtau_h, dtau_analytic);
    assert!(diff < 1e-8);

    // Compare Enzyme to analytic
    let dtau_enz = nh.d_stress(&e, &de);
    let diff = rel_diff(dtau_enz, dtau_analytic);
    assert!(diff < 1e-15);
}
