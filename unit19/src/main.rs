use hello_macro_derive::HelloMacro;
use unit19_hello_macro::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
    println!("Hello");

    let point = Point { x: 1, y: 2 };
    point.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");

    let mut w = Wrapper(vec!["hello".to_string(), "world".to_string()]);

    // 使用 Deref trait，像使用 Vec<String> 那样使用 Wrapper
    w.push("Rust".to_string());
    println!("{}", w);
}

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// newtype 模式用以在外部类型上实现外部 trait
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// 新类型拥有其内部类型的每一个方法，为封装类型实现 Deref trait
use std::ops::{Deref, DerefMut};
// 为 Wrapper 实现 Deref trait
impl Deref for Wrapper {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// 为 Wrapper 实现 DerefMut trait
impl DerefMut for Wrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
