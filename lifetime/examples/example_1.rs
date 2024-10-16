struct Manager<'a> {
    text: &'a str,
}
struct List<'a> {
    manager: Manager<'a>,
}
impl<'a> List<'a> {
    pub fn get_interface<'b>(&'b mut self) -> Interface<'b, 'a> {
        Interface {
            manager: &mut self.manager,
        }
    }
}
struct Interface<'a, 'b: 'a> {
    manager: &'a mut Manager<'b>,
}
impl<'a, 'b> Interface<'a, 'b> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

fn main() {
    let mut list: List<'_> = List {
        manager: Manager {
            text: "hello, world",
        },
    };
    list.get_interface().noop();

    println!("Interface should be consumed");

    use_list(&list);
}

fn use_list<'a>(list: &'a List<'a>) {
    println!("{}", list.manager.text);
}
