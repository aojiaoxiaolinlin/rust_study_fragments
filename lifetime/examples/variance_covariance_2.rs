fn call_with_static(func: fn(&'static str)) {
    func("hello"); // "hello" 是 'static 生命周期
}

fn process(s: &str) {
    println!("{}", s);
}

fn main() {
    // 逆变：如果 A 是 B 的子类型，则 T<B> 是 T<A> 的子类型（方向相反）。
    // 只有这一种情况，函数的参数是逆变的。F(T)->U 在 T 上逆变 但对U协变
    // process 接受任意生命周期（包括 'static），可以传递给需要 fn(&'static str) 的函数
    call_with_static(process); // 逆变允许传递
}
