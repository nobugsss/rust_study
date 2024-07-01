fn main() {
    let school = School {
        name: String::from("xxx学校"),
        age: 100,
    };
    school.do_sth(|age| {
        println!("age: {:?}", age);
    });

    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    // 试试删除类型说明
    let num: Number = int.into();
    println!("My number is {:?}", num);

    let circle = Circle { radius: 6 };
    println!("to_string:{}", circle.to_string());
    let circle_string = circle.to_string();
    println!("circle_string:{}", circle_string);
    assert_eq!(circle_string, String::from("Circle of radius 6"));

    // `enum` 可以转成整型。
    println!("zero is {}", Number2::Zero as i32);
    println!("one is {}", Number2::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

struct School {
    name: String,
    age: usize,
}

impl School {
    fn do_sth<F>(&self, f: F)
    where
        F: FnOnce(usize),
    {
        println!("name: {:?}", self.name);
        f(self.age)
    }
}

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

use std::fmt::{self, write};

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

// 拥有隐式辨别值（implicit discriminator，从 0 开始）的 enum
enum Number2 {
    Zero,
    One,
    Two,
}

// 拥有显式辨别值（explicit discriminator）的 enum
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}
