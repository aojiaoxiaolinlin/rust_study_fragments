use std::marker::PhantomData;

fn main() {
    let mut my_vec = Vec::new();
    let value = &10;
    insert_value(&mut my_vec, value);
}

fn insert_value<'a, 'b: 'a>(my_vec: &'a mut Vec<&'b i32>, value: &'b i32) {
    my_vec.push(value);
}

struct MyStruct<'a> {
    value: &'a i32,
}

impl MyStruct<'_> {
    fn new(value: &i32) -> MyStruct<'_> {
        MyStruct { value }
    }
}

struct Bar<'r> {
    // 逆变，
    _phantom: PhantomData<fn(&'r ())>,
}

fn bar<'short, 'long: 'static>(mut short_bar: Bar<'short>, mut long_bar: Bar<'long>) {
    // 'long是’short的子类型，所以Bar<'long>是Bar<'short>的父类型
    // short_bar = long_bar; // esrror here，父类型不能赋值给子类型
    long_bar = short_bar; // ok
}

fn bar2<'short, 'long: 'static>(short_bar: &mut Bar<'short>, long_bar: &mut Bar<'long>) {
    // &mut T 是不变的，所以&mut Bar<'long>是&mut Bar<'short>是没有任何关系的
    // short_bar = long_bar; // error here
    // long_bar = short_bar; // error here1
}

// 入参的生命周期一定是'static的
fn bar3<T>(_input: &'static T) {}

// T中的引用一定是'static的
fn bar4<T: 'static>(_input: &T) {}
