use macro_stu::{my_macro, my_macro2};
fn main() {
    my_macro!();
    let v = my_macro2!(1, 2, 3);
    println!("{:?}", v)
}
