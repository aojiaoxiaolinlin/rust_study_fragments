trait Animal {
    fn name(&self);
    fn age(&self);
}

struct Dog {
    name: String,
    age: u8,
}
impl Animal for Dog {
    fn name(&self) {
        println!("Dog name is {}", self.name);
    }
    fn age(&self) {
        println!("Dog age is {}", self.age);
    }
}
struct Cat {
    name: String,
    age: u8,
}
impl Animal for Cat {
    fn name(&self) {
        println!("Cat name is {}", self.name);
    }
    fn age(&self) {
        println!("Cat age is {}", self.age);
    }
}
struct Zoo {
    // 使用Box<dyn Trait>存储trait对象
    animals: Vec<Box<dyn Animal>>,
    // 使用impl Trait存储trait对象
    // animals2: Vec<impl Animal>, // error[E0308]: `impl Trait` not allowed outside of function and inherent method return types
}
fn main() {
    let dog = Dog {
        name: String::from("Tommy"),
        age: 5,
    };
    let cat = Cat {
        name: String::from("Kitty"),
        age: 3,
    };
    let zoo = Zoo {
        animals: vec![Box::new(dog), Box::new(cat)],
    };
    for animal in zoo.animals.iter() {
        animal.name();
        animal.age();
    }

    
}
// 通过impl Trait返回trait，静态分发，编译器会自动推断返回类型，性能较好
fn name() -> impl Animal {
    let dog = Dog {
        name: String::from("Tommy"),
        age: 5,
    };
    return dog;
}

// 通过Box<dyn Trait>返回trait，动态分发，编译器无法推断返回类型，性能较差
fn age() -> Box<dyn Animal> {
    let cat = Cat {
        name: String::from("Kitty"),
        age: 3,
    };
    return Box::new(cat);
}
