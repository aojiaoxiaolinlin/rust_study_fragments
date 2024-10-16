use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    sync::Arc,
};
#[derive(Debug)]
struct Cat {
    name: String,
}

impl From<Cat> for Arc<RefCell<Animal>> {
    fn from(cat: Cat) -> Self {
        Arc::new(RefCell::new(Animal::Cat(Arc::new(RefCell::new(cat)))))
    }
}
struct MulRef(Arc<RefCell<Cat>>);
impl From<MulRef> for Arc<RefCell<Animal>> {
    fn from(cat: MulRef) -> Self {
        Arc::new(RefCell::new(Animal::Cat(cat.0)))
    }
}
#[derive(Debug)]
struct Dog {
    name: String,
}

struct Show {
    name: String,
    list: Vec<Arc<RefCell<Animal>>>,
    depth_list: BTreeMap<u16, Arc<RefCell<Animal>>>,
}
impl Debug for Show {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name.clone();
        let mut list = String::new();
        recursive_show(&self.list, &mut list);

        write!(f, "Show: {}, list: {}", name, list)
    }
}

fn recursive_show(list: &Vec<Arc<RefCell<Animal>>>, result: &mut String) {
    for animal in list {
        animal.borrow_mut().update_name();
        let mut animal = animal.clone();
        let mut make_mut = Arc::make_mut(&mut animal);
        let get_mut = RefCell::get_mut(&mut make_mut);
        result.push_str(", ");
        match get_mut {
            Animal::Cat(cat) => result.push_str(&cat.borrow().name),
            Animal::Dog(dog) => result.push_str(&dog.borrow().name),
            Animal::Show(show) => {
                result.push_str(&show.borrow().name);
                recursive_show(&show.borrow().list, result);
            }
        }
    }
}

impl Show {
    fn insert_object(&mut self, id: u16, library: &Library) {
        if let Some(object) = library.object.get(&id) {
            match object {
                Object::Cat(cat) => {
                    // let cat = Arc::new(RefCell::new(Animal::Cat(cat.clone())));
                    let animal: Arc<RefCell<Animal>> = MulRef(cat.clone()).into();
                    self.list.push(animal.clone());
                    self.depth_list.insert(id, animal);
                }
                Object::Dog(dog) => {
                    let dog = Arc::new(RefCell::new(Animal::Dog(dog.clone())));
                    self.list.push(dog.clone());
                    self.depth_list.insert(id, dog);
                }
                Object::Show(show) => {
                    let show = Arc::new(RefCell::new(Animal::Show(show.clone())));
                    self.list.push(show.clone());
                    self.depth_list.insert(id, show);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Animal {
    Cat(Arc<RefCell<Cat>>),
    Dog(Arc<RefCell<Dog>>),
    Show(Arc<RefCell<Show>>),
}
impl Animal {
    fn update_name(&mut self) {
        self.set_name(self.get_name() + "我是大坏蛋");
    }
}

trait AnimalTrait: Debug {
    fn get_name(&self) -> String;
    fn set_name(&mut self, name: String);
}

impl AnimalTrait for Animal {
    fn get_name(&self) -> String {
        match self {
            Animal::Cat(cat) => cat.borrow().name.clone(),
            Animal::Dog(dog) => dog.borrow().name.clone(),
            Animal::Show(show) => show.borrow().name.clone(),
        }
    }
    fn set_name(&mut self, name: String) {
        match self {
            Animal::Cat(cat) => cat.borrow_mut().name = name,
            Animal::Dog(dog) => dog.borrow_mut().name = name,
            Animal::Show(show) => show.borrow_mut().name = name,
        }
    }
}

enum Object {
    Cat(Arc<RefCell<Cat>>),
    Dog(Arc<RefCell<Dog>>),
    Show(Arc<RefCell<Show>>),
}
struct Library {
    object: HashMap<u16, Object>,
}
fn main() {
    let cat = Arc::new(RefCell::new(Cat {
        name: "Kitty".to_string(),
    }));
    let dog = Arc::new(RefCell::new(Dog {
        name: "Doggy".to_string(),
    }));

    let show = Arc::new(RefCell::new(Show {
        name: "Show".to_string(),
        list: Vec::new(),
        depth_list: BTreeMap::new(),
    }));
    let mut library = Library {
        object: HashMap::new(),
    };
    library.object.insert(1, Object::Cat(cat.clone()));
    library.object.insert(2, Object::Dog(dog.clone()));
    show.borrow_mut().insert_object(1, &library);
    show.borrow_mut().insert_object(2, &library);
    library.object.insert(3, Object::Show(show.clone()));

    let mut show = Show {
        name: "Root Show".to_string(),
        list: Vec::new(),
        depth_list: BTreeMap::new(),
    };

    show.insert_object(1, &library);
    show.insert_object(2, &library);
    show.insert_object(3, &library);

    println!("修改前数据列 {:?}", show);

    let mut animal = show.depth_list.get_mut(&1).unwrap().borrow_mut();
    animal.set_name("洁大坏蛋".to_string());
    drop(animal);
    println!("修改后数据列 {:?}", show);
}
