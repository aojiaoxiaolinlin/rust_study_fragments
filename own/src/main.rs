#[derive(Debug, Clone, Copy)]
struct A<'a> {
    age: i32,
    name: &'a str,
}

fn main() {
    let a = String::from("Hello");
    let b = a.clone();
    let a = &a as *const String;
    let b = &b as *const String;
    println!("a = {:?}, b = {:?}", a, b);

    let a = A {
        age: 1,
        name: "Hello",
    };
    let b = a.clone();
    let a = &a as *const A;
    let b = &b as *const A;

    println!("a = {:?}, b = {:?}", a, b);

    let mut a = A {
        age: 1,
        name: "Hello",
    };
    a.name = "World";
    let b = a;
    println!("a = {:?}, b = {:?}", a, b);
    let a = &a as *const A;
    let b = &b as *const A;

    println!("a = {:?}, b = {:?}", a, b);
}
