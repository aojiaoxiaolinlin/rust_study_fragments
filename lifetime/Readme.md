# 生命周期

## 生命周期消除

1. 每一个引用参数都会获得独自的生命周期

```rust
fn foo<'a>(x: &'a i32){}
fn foo<'a, 'b>(x: &'a i32, y: &'b i32){}
```

2. 若只有一个输入生命周期(函数参数中只有一个引用类型)，那么该生命周期会被赋给所有的输出生命周期，也就是所有返回值的生命周期都等于该输入生命周期

```rust
fn foo(x: &i32) -> &i32{}
// 等同于
fn foo<'a>(x: &'a i32) -> &'a i32
```

3. 若存在多个输入生命周期，且其中一个是 &self 或 &mut self，则 &self 的生命周期被赋给所有的输出生命周期

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```
## 静态生命周期

拥有该生命周期的引用可以和整个程序活得一样久。

## 避免无界生命周期（使用unsafe时）

## 生命周期约束

'a:'b 表示 'a至少比'b活的一样久（'a>='b）

```rust
struct DoubleRef<'a,'b:'a, T> {
    r: &'a T,
    s: &'b T
}
```

T:'a 表示类型`T`必须比`'a`我的要久

```rust
struct Ref<'a, T: 'a> {
    r: &'a T
}

// 1.31版本开始使用下面写法
// 编译器自动推导T: 'a
struct Ref<'a, T> {
    r: &'a T
}
```
## NLL（Non-Lexical Lifetime）

引用的生命周期从借用处开始，一直持续到最后一次使用的地方

## Reborrow 再借用

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

fn main() {
    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;
    let rr: &Point = &*r;
    // 此时不能再rr生命周期内使用r
    println!("{:?}", rr);
    r.move_to(10, 10);
    println!("{:?}", r);
}
```

## &'static and T: 'static

`&'static`被引用的声明周期全局有效，且可以强制缩小生命周期
拥有所有权量的生命周期是`'static'`？
特征对象加+'static用来进行约束需要所有权？