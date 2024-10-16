#[derive(Debug)]
pub enum Node {
    Parent(Parent),
}
#[derive(Debug)]
struct Parent {
    child: Child,
    name: String,
    age: u32,
}
impl Parent {
    fn new(name: String, age: u32) -> Parent {
        Parent {
            child: Child::new(),
            name,
            age,
        }
    }
}

#[derive(Debug)]
struct Child {
    list: Vec<Node>,
}

impl Child {
    fn new() -> Child {
        Child { list: Vec::new() }
    }
    fn add(&mut self, node: Node) {
        self.list.push(node);
    }
}

fn main() {
    let mut parent = Parent::new("John".to_string(), 30);
    parent
        .child
        .add(Node::Parent(Parent::new("1".to_string(), 30)));
    parent
        .child
        .add(Node::Parent(Parent::new("2".to_string(), 30)));

    println!("{:?}", parent);
}
