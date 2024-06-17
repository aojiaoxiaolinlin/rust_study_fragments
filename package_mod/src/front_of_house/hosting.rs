pub fn add_to_waitlist() {
    println!("add_to_waitlist");
    // 调用serve_order
    super::serving::serve_order();
}

pub fn seat_at_table() {
    println!("seat_at_table");
}