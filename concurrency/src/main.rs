use std::{sync::mpsc, thread};
/// # 通过通信来共享内存
fn main() {
    // mspc 是多生产者单消费者的缩写
    let (tx, rx) = mpsc::channel();
    // 多生产者
    let tx1 = tx.clone();
    thread::spawn(move || {
        for i in 0..100 {
            tx.send(i).unwrap();
        }
    });
    thread::spawn(move || {
        for i in 100..201 {
            tx1.send(i).unwrap();
        }
    });
    let sum = rx.iter().take(200).sum::<i32>();
    // 用迭代器的方式接收发出的消息
    // for received in rx {}

    println!("sum: {}", sum);
}
