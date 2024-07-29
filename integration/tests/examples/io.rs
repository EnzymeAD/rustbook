#![feature(autodiff)]

#[no_mangle]
#[autodiff(diff, Reverse)]
fn eprintfunc() {
    eprintln!("eprintln");
    println!("println");
}

fn main() {
    diff();
}
