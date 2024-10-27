samples::test! {
    vec;
    use std::autodiff::autodiff;
    #[autodiff(jac, ReverseFirst, Duplicated, Duplicated)]
    fn sin(x: &Vec<f32>, y: &mut f32) {
        *y = x.into_iter().map(|x| f32::sin(*x)).sum()
    }

    #[autodiff(hessian, Forward, Dual, Dual, Const, Const)]
    fn jac2(x: &Vec<f32>, b_x: &mut Vec<f32>, y: &mut f32, b_y: &mut f32) {
        jac(x, b_x, y, b_y);
    }


    fn main() {
        let inp = vec![3.1415 / 2., 1.0, 0.5];
        let mut b_inp = vec![0.0, 0.0, 0.0];
        let mut db_inp = vec![0.0, 0.0, 0.0];
        let mut y = 0.0;
        let tang = vec![0.0, 1.0, 0.0];
        hessian(&inp, &tang, &mut b_inp, &mut db_inp, &mut y, &mut 1.0);
        assert_eq!(inp.iter().map(|x| x.sin()).sum::<f32>(), y);
        assert_eq!(inp.iter().map(|x| x.cos()).collect::<Vec<_>>(), b_inp);
        assert_eq!(vec![0.0, -inp[1].sin(), 0.0], db_inp);
    }
}
