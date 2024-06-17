use generic_and_trait::{notify, Weibo};
// 泛型并添加trait约束
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

struct Point<T> {
    x: T,
    y: T,
}
// 方法中使用
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
struct Point2<T, U> {
    x: T,
    y: U,
}
impl<T,U> Point2<T, U> {
    fn mixup<V,W>(self,other:Point2<V,W>) -> Point2<T,W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main(){
    let a = 1;
    let b = 2;
    let c = add(a,b);
    println!("{}",c);
    let a = 1.0;
    let b = 2.0;
    let c = add(a,b);
    println!("{}",c);

    let p = Point{x: 1, y: 2};
    println!("{} {}", p.x, p.y);
    let p = Point{x: 3.0, y: 4.0};
    println!("{} {}", p.x, p.y);

    println!("p.x = {}", p.x());

    let p1= Point2{x: 5, y: 10.4};
    let p2= Point2{x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("{} {}", p3.x, p3.y);

    let web: Weibo = Weibo {
        username: String::from("web"),
        content: String::from("hello")
    };

    notify(&web);

}