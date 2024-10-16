use std::{cell::RefCell, rc::Rc};

// 定义一个特征
trait Animal {
    fn make_sound(&self) -> String;
}

// 实现该特征的结构体
struct Dog;
impl Animal for Dog {
    fn make_sound(&self) -> String {
        "Woof!".to_string()
    }
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) -> String {
        "Meow!".to_string()
    }
}

// 使用泛型的函数,实现静态分发
fn animal_sound<T: Animal>(animal: T) -> String {
    animal.make_sound()
}
// 与前一个等价
fn animal_sound2(animal: impl Animal) -> String {
    animal.make_sound()
}
// 动态分发
fn animal_sound3(animal: Box<dyn Animal>) -> String {
    animal.make_sound()
}

fn animal_sound4(animal: &dyn Animal) -> String {
    animal.make_sound()
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    // println!("Dog says: {}", animal_sound(dog));
    // println!("Cat says: {}", animal_sound(cat));

    // println!("Dog says: {}", animal_sound2(dog));
    // println!("Cat says: {}", animal_sound2(cat));

    // println!("Dog says: {}", animal_sound3(Box::new(dog)));
    // println!("Cat says: {}", animal_sound3(Box::new(cat)));

    // println!("Dog says: {}", animal_sound2(dog));
    // println!("Cat says: {}", animal_sound2(cat));

    let box_a = Rc::new(RefCell::new(Box::new(1)));
    **box_a.borrow_mut() += 1;

    let box_b = box_a.clone();

    **box_b.borrow_mut() += 99;

    dbg!(box_a);
    dbg!(box_b);
}
