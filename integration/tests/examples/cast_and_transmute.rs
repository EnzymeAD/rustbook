#![feature(autodiff)]

#[no_mangle]
#[autodiff(b_dot_local, Reverse, Duplicated, Active)]
fn dot_local(x: &usize) -> f64 {
    // dereference x, and cast the usize back to the double ptr.
    // Then load a double from the pointer and return sin(the value).
    let x = unsafe { *x as *const f64 };
    let x = unsafe { *x };
    x.sin()
}

// now the same with std::mem::transmute
#[no_mangle]
#[autodiff(b_dot_local2, Reverse, Duplicated, Active)]
fn dot_local2(x: &usize) -> f64 {
    // dereference x, and std::mem::transmute the usize back to the double ptr.
    // Then load a double from the pointer and return sin(the value).
    let x = unsafe { std::mem::transmute::<usize, *const f64>(*x) };
    let x = unsafe { *x };
    x.sin()
}

fn main() {
    let x = 1.34;
    let mut dx = 0.0;
    let mut dx2 = 0.0;
    let x1: usize = &x as *const f64 as usize;
    let mut dx1: usize = &mut dx as *mut f64 as usize;
    let mut dx2: usize = &mut dx2 as *mut f64 as usize;
    b_dot_local(&x1, &mut dx1, 1.0);
    b_dot_local2(&x1, &mut dx2, 1.0);
    let dx1f = unsafe { *(dx1 as *const f64) };
    let dx2f = unsafe { *(dx2 as *const f64) };
    assert_eq!(dx1f, f64::cos(x));
    assert_eq!(dx2f, f64::cos(x));
}
