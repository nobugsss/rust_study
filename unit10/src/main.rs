fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_fn(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_fn(&number_list);
    println!("The largest number is {result}");

    // 10.1泛型数据类型
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_type(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_type(&char_list);
    println!("The largest char is {result}");

    // 结构体定义中的泛型
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4 };
    println!("integer: {:?}", integer);
    println!("float: {:?}", float);
    println!("integer.x = {}", integer.x);

    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    use unit10::{NewsArticle, Summary, Tweet}; // 导入需要的结构体和 trait
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    // 10.3. 生命周期确保引用有效
    let r;
    {
        let x = 5;
        r = &x;
        println!("r: {r}");
    }
    // println!("r: {r}");  // 报错

    // 报错：Rust 并不知道将要返回的引用是指向 x 或 y
    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

fn largest_fn(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_type<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 结构体定义中的泛型
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}
// 方法定义中的泛型
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}
struct Point2<T> {
    x: T,
    y: T,
}
// 单独为 Point<f32> 实例实现方法，而不是为泛型 Point 实例
impl Point2<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// // 枚举定义中的泛型
// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// 使用与结构体定义中不同类型的泛型
struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}
