use std::thread;
fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // let example_closure = |x| x;

    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5);

    // 定义并调用一个捕获不可变引用的闭包
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    // 定义并调用一个捕获可变引用的闭包
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);
    //   borrows_mutably 定义时，它捕获了 list 的可变引用.
    // 闭包在被调用后就不再被使用，这时可变借用结束。
    // 因为当可变借用存在时不允许有其它的借用，所以在闭包定义和调用之间不能有不可变引用来进行打印。
    borrows_mutably();
    println!("After calling closure: {list:?}");

    // 使用 move 来强制闭包为线程获取 list 的所有权
    let list = vec![1, 2, 33];
    println!("Before defining closure: {list:?}");

    // 去掉move报错
    // 新线程可能在主线程剩余部分执行完前执行完，或者也可能主线程先执行完。如果主线程维护了 list 的所有权但却在新线程之前结束并且丢弃了 list，则在线程中的不可变引用将失效。因此，编译器要求 list 被移动到在新线程中运行的闭包中，这样引用就是有效的。
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
    // println!("after move closure: {list:?}"); // 报错

    // 获取 list 的所有权2
    let list2 = vec![1, 2, 3];
    println!("Before defining closure: {list2:?}");

    let only_borrows = || {
        let mut li = list2;
        li.push(33);
        println!("From closure111: {li:?}")
    };
    // 报错 list已指针已被移动到闭包中的变量li, list 已无效
    // println!("Before calling closure: {list2:?}");
    only_borrows();
    // println!("After calling closure: {list2:?}");

    // 将被捕获的值移出闭包和 Fn trait

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");

    // 尝试在 sort_by_key 上使用一个 FnOnce 闭包
    // 该代码尝试在闭包的环境中向 sort_operations vector 放入 value— 一个 String 来实现计数。
    // 包捕获了 value 然后通过转移 value 的所有权的方式将其移出闭包给到 sort_operations vector。
    // 这个闭包可以被调用一次，尝试再次调用它将报错。因为这时 value 已经不在闭包的环境中，无法被再次放到 sort_operations 中
    // 因而，这个闭包只实现了 FnOnce
    // let mut list = [
    //     Rectangle {
    //         width: 10,
    //         height: 1,
    //     },
    //     Rectangle {
    //         width: 3,
    //         height: 5,
    //     },
    //     Rectangle {
    //         width: 7,
    //         height: 12,
    //     },
    // ];

    // let mut sort_operations = vec![];
    // let value = String::from("closure called");

    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });
    // println!("{list:#?}");

    // 报错指向了闭包体中将 value 移出环境的那一行。
    // 要修复这里，我们需要改变闭包体让它不将值移出环境。

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");

    // 13.2. 使用迭代器处理元素序列
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }
    // println!("v1_iter: {:?}", v1_iter);

    // 调用 map 方法创建一个新迭代器，接着调用 collect 方法消费新迭代器并创建一个 vector
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
// enum Option<T> {
//     Some(T),
//     None,
// }
// impl<T> Option<T> {
//     pub fn unwrap_or_else<F>(self, f: F) -> T
//     where
//         F: FnOnce() -> T,
//     {
//         match self {
//             Some(x) => x,
//             None => f(),
//         }
//     }
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 13.2. 使用迭代器处理元素序列
