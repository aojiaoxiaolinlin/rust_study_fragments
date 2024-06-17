use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, Copy)]
struct TestCopy {
    a: i32,
    b: i32,
}
#[derive(Debug, Clone)]
struct TestClone {
    a: i32,
    b: i32,
}
fn main() {
    let mut a = TestCopy { a: 1, b: 2 };
    let b = a;
    a.a = 3;
    println!("a = {:?}, b = {:?}", a, b);
    println!("a = {:?}, b = {:?}", &a as *const TestCopy, &b as *const TestCopy);

    let mut a = TestClone { a: 1, b: 2 };

    let b = a.clone();

    a.a = 3;
    println!("a = {:?}, b = {:?}", a, b);

    println!("{:?}", &a as *const TestClone);
    println!("{:?}", &b as *const TestClone);
    let c = &a;
    println!("{:?}", c as *const TestClone);

    let string = Rc::new(RefCell::new(String::from("hello")));
    let string1 = Rc::clone(&string);
    let string2 = Rc::clone(&string);

    println!("{:?}", string1.as_ptr());
    println!("{:?}", string2.as_ptr());
}
