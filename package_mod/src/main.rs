use package_mod::front_of_house;
fn main() {
    front_of_house::hosting::add_to_waitlist();

    for i in 0..3 {
        let x = (1 - i) as f32 * 0.5;
        let y = ((i & 1) * 2 - 1) as f32 * 0.5;
        println!("x: {}, y: {}", x, y);
    }
    let a = 1u32;
    let b = 2i32;
}
