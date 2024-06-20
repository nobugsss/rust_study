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

    println!("New article available! {}", article.summarize_author());

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

    // 生命周期引用
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // 传递拥有不同具体生命周期的引用来限制 longest 函数的使用
    let string3 = String::from("long string is long!");
    {
        let string4 = String::from("xyz");
        let result = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {result}");
    }

    // 该例子揭示了 result 的引用的生命周期必须是两个参数中较短的那个
    // longest 函数返回的引用的生命周期应该与传入参数的生命周期中较短那个保持一致,所以下面的代码无法编译
    // let string5 = String::from("long string is long");
    // let result;
    // {
    //     let string6 = String::from("xyz");
    //     result = longest(string5.as_str(), string6.as_str());
    // }
    // println!("The longest string is {result}");

    // 结构体定义中的生命周期注解结构体定义中的生命周期注解
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("i.part: {}", i.part);

    // 方法定义中的生命周期注解
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }
    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announce_and_return_part: &str) -> &str {
            println!("Attention please: {announce_and_return_part}");
            self.part
        }
    }
    println!("i.levle: {:?}", i.level());
    println!(
        "i.announce_and_return_part: {:?}",
        i.announce_and_return_part("hello")
    );

    // 静态生命周期
    let ss: &'static str = "I have a static lifetime."; //'static，其生命周期能够存活于整个程序期间。所有的字符串字面值都拥有 'static 生命周期
    println!("ss: {:?}", ss);
    // 结合泛型类型参数、trait bounds 和生命周期
    // 在同一函数中指定泛型类型参数、trait bounds 和生命周期的语法！
    // 下面的代码留到后面研究
    // use std::fmt::Display;
    // fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    // where
    //     T: Display,
    // {
    //     println!("Announcement! {ann}");
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }
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
