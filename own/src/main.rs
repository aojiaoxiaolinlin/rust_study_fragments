#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct A<'a> {
    age: i32,
    name: &'a str,
}

fn main() {
    let a = String::from("Hello");
    let mut b = a.clone();
    b.push_str(", world");
    println!("a = {:?}, b = {:?}", a, b);
    let a = &a as *const String;
    let b = &b as *const String;
    assert_ne!(a, b);

    let a = A {
        age: 1,
        name: "Hello",
    };
    let b = a.clone();
    let a = &a as *const A;
    let b = &b as *const A;

    assert_ne!(a, b);

    let mut a = A {
        age: 1,
        name: "Hello",
    };
    a.name = "World";
    let b = a;
    assert_eq!(a, b);
    let a = &a as *const A;
    let b = &b as *const A;

    assert_ne!(a, b);
}
