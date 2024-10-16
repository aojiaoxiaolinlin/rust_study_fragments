use std::rc::Rc;
#[derive(Debug, PartialEq)]
struct Data {
    st: String,
    num: u8,
}

fn main() {
    let a = Rc::new(Data {
        st: String::from("hello"),
        num: 1,
    });
    let b = Rc::clone(&a);
    assert_eq!(a, b);
}
