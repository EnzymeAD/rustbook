#![feature(autodiff)]

//#[Derive(Debug, Clone, PartialEq)]
//#[autodiff(d_sum, Forward, Dual, Dual)]

#[autodiff(ds_sum, Reverse, Duplicated, Active)]
fn sum2(x: &[f32]) -> f32 {
    3.14
}

#[autodiff(d_sum, Reverse, Duplicated, Active)]
fn sum(x: &Vec<Vec<f32>>) -> f32 {
    x.into_iter().map(|x| x.into_iter().map(|x| x.sqrt())).flatten().sum()
}

fn main() {
    let a = vec![vec![1.0, 2.0, 4.0, 8.0]];
    //let mut b = vec![vec![0.0, 0.0, 0.0, 0.0]];
    let mut b = vec![vec![0.0, 0.0, 0.0, 0.0]];

    dbg!(&d_sum(&a, &mut b, 1.0));

    dbg!(&b);
}

#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        super::main()
    }
}
