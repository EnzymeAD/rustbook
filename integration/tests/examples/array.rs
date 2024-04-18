samples::test! {
    reverse_duplicated_active;
    #[autodiff(d_array, Reverse, Duplicated, Active)]
    fn array(arr: &[[[f32; 2]; 2]; 2]) -> f32 {
        arr[0][0][0] * arr[1][1][1]
    }

    fn main() {
        let arr = [[[2.0, 1.0], [1.0, 1.0]], [[1.0, 1.0], [1.0, 3.0]]];
        let mut b_arr = [[[0.0; 2]; 2]; 2];

        let y = d_array(&arr, &mut b_arr, 1.0);
        assert_eq!(6.0, y);
        assert_eq!([[[3.0, 0.0], [0.0; 2]], [[0.0; 2], [0.0, 2.0]]], b_arr);
    }
}
