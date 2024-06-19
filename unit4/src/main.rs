fn main() {
    // let s = "hello";
    // let s = String::from("hello");

    // let mut s = String::from("hello");
    // s.push_str(", world.");
    // println!("{}", s);

    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("s1={}", s1);

    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1={}, s2={}", s1, s2);

    // let x = 5;
    // let y =x;
    // println!("x={}, y={}", x, y);

    // let s = String::from("hello");
    // takes_ownership(s);
    // // println!("s={}", s);

    // let x = 5;
    // makes_copy(x);
    // println!("x={}", x);

    // let s1 = gives_ownership();
    // println!("s1={}", s1);
    // let s2 = String::from("hello");
    // println!("s2={}", s2);
    // let s3 = takes_and_gives_bake(s2);
    // println!("s3={}", s3);

    // let s1 = String::from("hello");
    // let (s2, length) = calculate_length(s1);
    // println!("s2={}, length={}", s2, length);

    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("s1={}, len={}", s1, len);

    // let mut s1 = String::from("hello");
    // change(&mut s1);
    // println!("s1={}", s1);

    // let mut s = String::from("hello");
    // let s1 = &s;
    // let s2 = &s;
    // let s3 = &mut s;
    // println!("s1={}, s2={}, s3={}", s1, s2, s3);

    // let mut s = String::from("hello");
    // let s1 = &s;
    // let s2 = &s;
    // println!("s1={}, s2={}", s1, s2);
    // let s3 = &mut s;
    // println!("s3={}", s3);

    // let reference_to_nothing = dangle();

    // let s = String::from("hello world");
    // let i = first_word(&s);
    // println!("i={}", i);

    // let s = String::from("hello world");
    // let word = first_word(&s);
    // println!("word={}", word);

    // let a = [1, 2, 3, 4, 5];
    // let b = a;
    // println!("a={}, b={}", a, b);

    let my_string = String::from("hello world");

    // `first_word` 适用于 `String`（的 slice），部分或全部
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` 也适用于 `String` 的引用，
    // 这等价于整个 `String` 的 slice
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` 适用于字符串字面值，部分或全部
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值已经 **是** 字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word(my_string_literal);
}

// fn takes_ownership(some_string: String) {
//     println!("some_string={}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("some_integer={}", some_integer);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("yours");
//     some_string
// }

// fn takes_and_gives_bake(a_string: String) -> String {
//     a_string
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let len = s.len();
//     (s, len)
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world.");
// }

// fn dangle() -> &String {
//     let s = String::from("hello");
//     // &s
//     // s
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

fn first_word(s: &str) -> &str {
    s
}
