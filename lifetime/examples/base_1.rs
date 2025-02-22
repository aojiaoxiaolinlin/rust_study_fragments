#[derive(Debug)]
struct Student<'a> {
    name: &'a str,
    age: i8,
}

impl Student<'_> {
    fn new_one(age: i8) -> Self {
        Student { name: "GG", age }
    }
}

impl<'a, 'b: 'a> Student<'a> {
    fn new(age: i8) -> Self {
        Student { name: "GG", age }
    }

    fn new_ref(name: &'a str, age: i8) -> Self {
        Student { name, age }
    }
}

fn main() {
    let student = Student {
        name: "lin",
        age: 18,
    };

    println!("{:?}", student);

    let res = call_with(exec, "  霖霖  ");
    println!("{res}")
}

fn call_with<'a, F>(func: F, data: &'a str) -> &'a str
where
    for<'b> F: Fn(&'b str) -> &'b str,
{
    func(data)
}

fn exec(data: &str) -> &str {
    data.trim()
}
