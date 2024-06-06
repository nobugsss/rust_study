fn main() {
    // 6.1定义枚举(TODO:懂的比较模糊，先看后面的内容再回头看)

    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let home = IpAddrKind::V4(127, 0, 0, 1);
    // let loopback = IpAddrKind::V6(String::from("::1"));
    // println!(" home={:?}", home);
    // println!(" loopback={:?}", loopback);

    // enum Message {
    //     Quit,
    //     Move{x: i32, y: i32},
    //     Write(String),
    //     ChangeColor(i32, i32, i32),
    // }

    // impl Message {
    //     fn call(&self) {}
    // }

    // let m = Message::Write(String::from("Hello"));
    // m.call();

    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    // let some_number = Some(1);
    // let some_chat = Some("A");
    // let absent_number = Option<i32> = None;

    // 6.2 匹配控制流构造

    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    // let dict_roll = 9;
    // match dict_roll {
    //     3 => add_fancy_hattan(),
    //     7 => remove_fancy_hattan(),
    //     other => move_player(other),
    // }

    // let dict_roll = 9;
    // match dict_roll {
    //     3 => add_fancy_hattan(),
    //     7 => remove_fancy_hattan(),
    //     _ => reroll(),
    // }

    // 6.3 简洁的控制流程if let
}

// 6.1定义枚举

// #[derive(Debug)]
// enum IpAddrKind {
//     // V4(String),
//     // V6(String),
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// struct QuitMessage;
// struct MoveMessage {
//     x: i32,
//     y: i32,
// };
// struct WriteMessage(String);
// struct ChangeColorMessage(i32, i32, i32)

// 6.2 匹配控制流构造;
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 16,
//     }
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// fn add_fancy_hat() {};
// fn remove_fancy_hat() {};
// fn move_player(num_spaces: u8) {};
// fn reroll(){};

// 6.3 简洁的控制流程if let
