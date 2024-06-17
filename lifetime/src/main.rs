use std::fmt::Debug;

fn print_it( input: impl Debug) {
    println!( "'static value passed in is: {:?}", input );
}
#[derive(Debug)]
struct B<'a> {
    name: &'a [u8],
}
#[derive(Debug)]
struct A<'a> {
    age: i32,
    name: &'a B<'a>,
}
fn main() {
    let a_str = "I'm a &str";
    // i 是拥有所有权的，不包含引用，因此它的生命周期是 'static
    let mut i = 5;
    print_it(i);
    i += 1;
    // 糟糕，&i 只有在 main() 作用域内定义的生命周期，因此它不是 'static：
    print_it(i);

    print_it(a_str);

    let test = test();

    println!("{:?}", test);
}

fn test() -> &'static mut A<'static> {
    let name = B{name: b"Hello, World!"};
    let name = Box::leak(Box::new(name));
    let a = A{
        name,
        age: 18,
    };
    Box::leak(Box::new(a))
}