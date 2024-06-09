fn main() {
    // 9.1 用 panic! 处理不可恢复的错误
    // panic!("crash and burn");

    // 使用 panic! 的 backtrace
    // let v = vec![1, 2, 3, 4, 5];
    // v[99];

    // 9.2 用 Result 处理可恢复的错误
    use std::fs::File;
    use std::io::ErrorKind;
    let greeting_file_result = File::open("hello.txt");
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {error:?}"),
    // };

    // 匹配不同的错误
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => panic!("Problem opening the file: {other_error:?}"),
        },
    };

    // 使用闭包和 unwrap_or_else改造上面很多match的情况，使代码简洁
    let greeting_file_closure = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // // 失败时 panic 的简写：unwrap 和 expect
    // let greeting_file3 = File::open("hello3.txt").unwrap(); // 没参数
    // let greeting_file4 =
    //     File::open("hello4.txt").expect("hello4.txt should be included in this project");
    // // 有参数

    // 传播错误
    use std::io::{self, Read};
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello5.txt");
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(err) => return Err(err),
        };
        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(err) => Err(err),
        }
    }

    // 传播错误的简写：? 运算符
    fn read_username_from_file2() -> Result<String, io::Error> {
        let mut username_file = File::open("hello6.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
    // or
    fn read_username_from_file3() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello7.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }

    // use std::fs;
    // use std::io;

    // fn read_username_from_file() -> Result<String, io::Error> {
    //     fs::read_to_string("hello.txt")
    // }
}
