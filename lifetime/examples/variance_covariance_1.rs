struct Container<'a> {
    data: &'a i32,
}

fn print_container(c: Container<'_>) {
    println!("{}", c.data);
}

fn main() {
    let x = 42;
    // 协变：若A是B的子类型，则 T<A> 是 T<B> 的子类型。

    // 参考https://nomicon.purewhite.io/subtyping.html
    // 这里是第一条：&'a T 在 ’a 和 T上协变
    // 'long 的生命周期（作用域内）包含 'short
    let long_container = Container { data: &x };
    print_container(long_container); // 协变允许传递
}
