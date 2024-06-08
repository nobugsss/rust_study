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

    // let data = "initial contents";
    // let s2 = data.to_string();
    // // or
    // let s2 = "initial contents".to_string();
    // // or
    // let s2 = String::from("initial contents");

    // let mut s = String::from("foo");
    // s.push_str("bar");
    // println!("s2: {}", s2);

    // let s3 = String::from("Hello");
    // let s4 = String::from("world");
    // let s5 = s3 + &s4 + "!"; // s3 被移动了，不能继续使用; 不能s3 + s4;
    // println!("s5: {}", s5);

    // let s6 = "a";
    // let s7 = "b";
    // let s8 = "c";
    // let s9 = format!("{s6}-{s7}/{s8}"); // 宏 format! 生成的代码使用引用所以不会获取任何参数的所有权
    // println!("s9={}", s9);

    // let s10 = "Hello";
    // println!("s10的长度={}", s10.len());
    // let s11 = "नमस्ते";
    // let n = '你';
    // let s12: &str = "你";
    // println!("s11的长度={}", s11.len()); // 返回字节数
    // println!("s11={:#?}", s11.chars());
    // println!("s12字节={:#?}", s12.bytes());
    // println!("s10字节={:?}", s10.bytes());

    // // 三种相关方式可以理解字符串：字节、标量值和字形簇（最接近人们眼中 字母 的概念）

    // 8.3. 使用 Hash Map 储存键值对
    // 哈希 map 将它们的数据储存在堆上，这个 HashMap 的键类型是 String 而值类型是 i32。
    // 类似于 vector，哈希 map 是同质的：所有的键必须是相同类型，值也必须都是相同类型。
    use std::collections::HashMap;
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert("Yellow".to_string(), 50);
    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);
    // println!("score: {:?}", score);
    // // let scores2 = HashMap::new();
    // // scores2.insert(1, 10);
    // // scores2.insert(2, 50);
    // for (key, value) in &scores {
    //     println!("key={}, value={}", key, value);
    // }

    // 对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map。对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者
    let filed_name = String::from("Favorite color");
    let filed_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(filed_name, filed_value);
    // 这里 field_name 和 field_value 不再有效，
    // 尝试使用它们看看会出现什么编译错误！
    // 如果将值的引用插入哈希 map，这些值本身将不会被移动进哈希 map。但是这些引用指向的值必须至少在哈希 map 有效时也是有效的

    let num = 1;
    let mut scores2 = HashMap::new();
    scores2.insert(num, 10);
    let val = scores2.get(&1).copied().unwrap_or(0);
    println!("val: {:?}", val);
    println!("num: {:?}", num);

    // 覆盖一个值
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    // 只在键没有对应值时插入键值对
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:?}");

    // 根据旧值更新一个值
    let text = "Hello world wonderful world";
    let mut word_map = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{word_map:?}");
}
