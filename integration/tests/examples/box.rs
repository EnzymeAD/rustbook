#![feature(autodiff)]

//#[autodiff(cos_box, Reverse, Active, Duplicated)]
#[autodiff(cos_box, Reverse, Duplicated, Active)]
fn sin(x: &Box<f32>) -> f32 {
    f32::sin(**x)
}

fn main() {
    let x = Box::<f32>::new(3.14);
    let mut df_dx = Box::<f32>::new(0.0);
    cos_box(&x, &mut df_dx, 1.0);

    dbg!(&df_dx);

    assert!(*df_dx == f32::cos(*x));
}

#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        super::main()
    }
}
