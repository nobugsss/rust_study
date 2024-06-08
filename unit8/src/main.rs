fn main() {
    // 8.1 使用Vector储存列表
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1,2,3,4,5];

    // let mut v = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);

    // println!("v = {:?}", v);

    // let v = vec![1, 2, 3, 4, 5];
    // let third = &v[2]; // 超出边间会报错
    // println!("third={}", third);
    // let third2 = v.get(3);
    // match third2 {
    //     Some(n) => println!("third={}", n),
    //     None => println!("third=None"),
    // }

    // // 不能在相同作用域中同时存在可变和不可变引用
    // let mut v = vec![1, 2, 3, 4];
    // let first = &v[0];
    // v.push(5);
    // println!("first={}", first);
    // // 允许
    // let mut v = vec![1, 2, 3, 4];
    // let first = &v[0];
    // println!("first={}", first);
    // v.push(5);

    // let v = vec![1, 2, 3, 4];
    // for i in v {
    //     println!("i={}", i);
    // }

    // // 修改可变引用所指向的值，在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值
    // let mut v = vec![1, 2, 3, 4];
    // for i in &mut v {
    //     *i += 10;
    // }
    // println!("v: {:?}", v);

    // // 使用枚举来储存多种类型
    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }
    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("Hello")),
    //     SpreadsheetCell::Float(8.88),
    // ];

    // 8.2 使用字符串储存 UTF-8 编码的文本
    // let mut s = String::new();

    let data = "initial contents";
    let s2 = data.to_string();
    // or
    let s2 = "initial contents".to_string();
    // or
    let s2 = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s2: {}", s2);

    let s3 = String::from("Hello");
    let s4 = String::from("world");
    let s5 = s3 + &s4 + "!"; // s3 被移动了，不能继续使用; 不能s3 + s4;
    println!("s5: {}", s5);
}
