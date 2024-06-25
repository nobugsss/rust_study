use std::mem::{size_of, size_of_val};

trait Greet<T> {
    fn greet(&self) -> T;
}

struct Person<T> {
    x: T,
}

impl<T: std::fmt::Display + Clone> Greet<T> for Person<T> {
    fn greet(&self) -> T {
        self.x.clone()
    }
}

fn main() {
    let person = Person {
        x: String::from("hello"),
    };
    let person2 = Person { x: 1 };
    println!("person.greet: {}", person.greet());
    println!("person2.greet: {}", person2.greet());

    // fn notify<T: Greet>(item1: &T, item2: &T) {
    //     item1.greet();
    //     item2.greet();
    // }
    // let x = "x";
    // let y: &str = &x;
    // println!("y:{}", x);

    // fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }

    // let r: &'a i32;

    // {
    //     let x: i32 = 5;
    //     r = &x;
    // }

    // println!("r: {r}");
    struct Point;
    let x = String::from("hello world");
    let v = vec![1, 2, 3, 4];
    let x1 = &x;
    let x2 = &x1;
    let x3 = &x2;
    let x4 = x1;
    let s = "hello world";

    println!("String: {}", size_of::<String>());
    println!("vec: {}", size_of::<Vec<i32>>());
    println!("&String: {}", size_of::<&String>());
    println!("point: {}", size_of::<&Point>());
    println!("&str: {}", size_of::<&str>());
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.into_iter();
    assert_eq!(v1_iter.next(), Some(1));

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    let b: Box<i32> = Box::new(5);
    println!("b = {b}");
    println!("Box: {}", size_of::<Box<i32>>());

    let mut s = String::from("hello");

    let r1 = &mut s;
    println!("{}", r1);
    let r2 = &mut s;

    // println!("{:?}", v1_iter);

    // println!("v1: {:?}", v1);

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    println!("valid coin: {}", value_in_cents(Coin::Penny));

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
    for i in 1..5 {
        println!("i: {:?}", i)
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    // println!("a: {:?} b: {:?}", a, b)
}

// trait Greet {
//     fn greet(&self) {
//         println!("Hello from the default implementation!");
//     }

//     fn greet_with_message(&self, message: &str) {
//         println!("{}", message);
//         self.greet();
//     }
// }

// struct Person;

// impl Greet for Person {
//     fn greet(&self) {
//         println!("Hello from Person!");
//     }

//     fn greet_with_message(&self, message: &str) {
//         println!("Person says: {}", message);
//         // 调用 trait 的默认 greet 方法而不是 Person 的 greet 方法
//         Greet::greet(self);
//     }
// }

// fn main() {
//     let person = Person;
//     person.greet_with_message("A custom message");
// }
