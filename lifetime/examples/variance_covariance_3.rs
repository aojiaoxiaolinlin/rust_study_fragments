fn overwrite<T>(input: &mut T, new_value: T) {
    *input = new_value;
}

fn main() {
    // 不变： 类型没有子类型关系。
    // 可变应用 &mut T
    let mut x = 42;
    let ref_mut: &mut i32 = &mut x;
    overwrite(ref_mut, 99); // 安全
}
