#![feature(autodiff)]

#[derive(Debug, PartialEq)]
enum Foo {
    A(f32),
    B(i32), 
}

#[autodiff(d_bar, Reverse, Duplicated, Active)]
fn bar(x: &Foo) -> f32 {








    match x  {
        Foo::A(f) => f * f,
        Foo::B(_) => 4.0,
    }
}

fn main() {
    let x = Foo::A(1.0);
    let x2 = Foo::A(-1.0);
    let x3 = Foo::B(1);
    let x4 = Foo::B(1);
    let x5 = Foo::A(1.0);
    let mut dx = Foo::A(0.0);
    let mut dx2 = Foo::A(0.0);
    let mut dx3 = Foo::B(0);
    let mut dx4 = Foo::A(0.0);
    let mut dx5 = Foo::B(0);
    let out = bar(&x);
    let dout = d_bar(&x, &mut dx, 1.0);
    let dout2 = d_bar(&x2, &mut dx2, 1.0);
    let dout3 = d_bar(&x3, &mut dx3, 1.0);
    let dout4 = d_bar(&x4, &mut dx4, 1.0);
    let dout5 = d_bar(&x5, &mut dx5, 1.0);
    println!("x: {out}");
    println!("dx: {dout}");
    println!("dx2: {dout2}");
    println!("dx3: {dout3}");
    println!("dx4: {dout4}");
    println!("dx5: {dout5}");
    assert_eq!(dx, Foo::A(2.0));
    assert_eq!(dx2, Foo::A(-2.0));
    assert_eq!(dx3, Foo::B(0));
    assert_eq!(dx4, Foo::A(0.0));
    assert_eq!(dx5, Foo::B(0));
}

