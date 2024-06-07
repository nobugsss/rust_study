// use std::fmt;
// use std::io;

// use std::fmt::Result;
// use std::io::Result as IoResult;  // 有相同的Result类型不用as会报错，或用上面的写法

mod front_of_house;

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         // fn seat_at_table() {}
//     }
//     // mod serving {
//     //     fn take_order() {}

//     //     fn serve_order() {}

//     //     fn take_payment() {}
//     // }
// }

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // TODO: 深入&str String
        pub fn summer(toast: &str) -> Self {
            Self {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
        // pub fn summer(toast: &str) -> Breakfast {
        //     Breakfast {
        //         toast: String::from(toast),
        //         seasonal_fruit: String::from("peaches"),
        //     }
        // }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // // 绝对路径
    // crate::front_of_house::hosting::add_to_waitlist();

    // // 相对路径
    // front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果取消下一行的注释代码不能编译；
    // 不允许查看或修改早餐附带的季节水果
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
