pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

pub fn greeting2(name: &str) -> String {
    format!("Hello!")
}

// 私有也可测试
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
    pub fn new2(value: i32) -> Guess {
        // if value < 1 {
        //     panic!("Guess value must be greater than or equal to 1, got {value}.");
        // } else if value > 100 {
        //     panic!("Guess value must be less than or equal to 100, got {value}.");
        // }

        // 测试确实如期望 panic 了，不过 panic 信息中并没有包含 expected 信息 'Guess value must be less than or equal to 100'。
        // 而我们得到的 panic 信息是 'Guess value must be greater than or equal to 1, got 200.'。
        if value < 1 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        // 检查是否相等
        assert_eq!(result, 4); // 相等
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // #[test]
    // fn it_adds_two() {
    //     assert_eq!(6, add_two(2));
    //     assert_ne!(6, add_two(2)); // left 和 right不相等才通过
    // }

    // 自定义失败信息
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
    // #[test]
    // fn greeting_contains_name2() {
    //     let result = greeting2("Carol");
    //     assert!(
    //         result.contains("Carol"),
    //         "Greeting did not contain name, value was `{}`",
    //         result
    //     );
    // }

    // 使用 should_panic 检查 panic
    // 检查代码是否按照期望处理错误
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
    // #[test]
    // #[should_panic(expected = "less than or equal to 100")]
    // fn greater_than_100_err() {
    //     Guess::new2(200);
    // }

    // 将 Result<T, E> 用于测试
    #[test]
    fn test_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // 测试私有函数
    #[test]
    fn internal() {
        assert_eq!(6, internal_adder(4, 2));
    }
}
