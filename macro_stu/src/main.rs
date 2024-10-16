use macro_stu::{my_macro, my_macro2, HelloMacro};

use hello_macro_derive::HelloMacro;
#[derive(HelloMacro)]
struct Sunfei;

#[derive(HelloMacro)]
struct Sunface;

fn main() {
    my_macro!();
    let v = my_macro2!(1, 2, 3);
    println!("{:?}", v);

    Sunfei::hello_macro();
    Sunface::hello_macro();
}
