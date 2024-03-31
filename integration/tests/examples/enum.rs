#![feature(autodiff)]

enum Foo {
    A(f32),
    B(i32), 
}

#[autodiff(d_bar, Reverse, Duplicated, Active)]
fn bar(x: &f32) -> f32 {
    let val: Foo = 
    if *x > 0.0 {
        Foo::A(*x)
    } else {
        Foo::B(12)
    };

    std::hint::black_box(&val);
    match val  {
        Foo::A(f) => f * f,
        Foo::B(_) => 4.0,
    }
}

fn main() {
    let x = 1.0;
    let x2 = -1.0;
    let mut dx = 0.0;
    let mut dx2 = 0.0;
    let out = bar(&x);
    let dout = d_bar(&x, &mut dx, 1.0);
    let dout2 = d_bar(&x2, &mut dx2, 1.0);
    println!("x: {out}");
    println!("dx: {dout}");
    println!("dx2: {dout2}");
    assert_eq!(dx, 2.0);
    assert_eq!(dx2, 0.0);
}

