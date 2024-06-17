#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

#[macro_export] // 将宏导出到外部作用域
macro_rules! my_macro2 {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);)*
            temp_vec
        }
    };
}