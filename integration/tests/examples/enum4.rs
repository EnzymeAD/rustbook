#![feature(autodiff)]

struct Bar {
    x: f32,
    y: bool,
}

#[autodiff(df, Reverse, Duplicated, Duplicated, Active)]
fn f(x: &Bar, val: bool) -> f32 {
    if val {
        x.x
    } else {
        4.0
    }
}

fn main() {
    //let a = Bar { x: 1.0, y: true };
    //let mut da_good = Bar { x: 0.0, y: true };
    //let mut da_bad = Bar { x: 0.0, y: false };
    //let dx = df(&a, &mut da_good, 1.0);
    //let dx2 = df(&a, &mut da_bad, 1.0);
    //println!("good: {:?}", da_good.x);
    //println!("bad: {:?}", da_bad.x);
    //println!("bool values good/bad: {:?} {:?}", da_good.y, da_bad.y);
}
