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

    fn notify<T: Greet>(item1: &T, item2: &T) {
        item1.greet();
        item2.greet();
        // if item2.greet() == 1 {
        //     item2.greet()
        // } else {
        //     item1.greet()
        // }
    }
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
