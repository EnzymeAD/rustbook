samples::test! {
    duplicated_active;
    use std::autodiff::autodiff;
    #[autodiff(cos_box, Reverse, Duplicated, Active)]
    fn sin(x: &Box<f32>) -> f32 {
        f32::sin(**x)
    }

    fn main() {
        let x = Box::<f32>::new(3.14);
        let mut df_dx = Box::<f32>::new(0.0);
        let y = cos_box(&x, &mut df_dx, 1.0);
        assert_eq!(f32::sin(*x), y);
        assert_eq!(f32::cos(*x), *df_dx);
    }
}
