fn assign<T>(input: &mut T, val: T) {
    *input = val;
}

fn main() {
    let mut hello: &'static str = "hello";
    {
        let world = String::from("world");
        // assign(&mut hello, &world);
    }
    println!("{hello}");
}
/*
error[E0597]: `world` does not live long enough
    --> lifetime\examples\base_2.rs:9:28
    |
    6  |     let mut hello: &'static str = "hello";
    |                    ------------ type annotation requires that `world` is borrowed for `'static`
    7  |     {
    8  |         let world = String::from("world");
    |             ----- binding `world` declared here
    9  |         assign(&mut hello, &world);
    |                            ^^^^^^ borrowed value does not live long enough
    10 |     }
    |     - `world` dropped here while still borrowed

    解释：
    首先 &mut T 对 T 是不变的 所有要求入参hello的生命周期也是&'static，但是又由于两个入参的生命周期要求一样
    所以就会要求入参world的生命周期也是&’static 的，但是又由于&'a T 在‘a上协变，
    所以编译器又会要求world的生命周期必须是&'static 的子类型。也就是说要'world的生命周期长于&'static。这显然不成立，
    因此编译器报错。
*/
