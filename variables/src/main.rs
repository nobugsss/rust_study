// use std::io;
fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}")


    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }
    // println!("The value of x is: {x}")

    // let spaces = "      ";
    // let spaces = spaces.len();

    // let mut spaces = "      ";
    // spaces = spaces.len();

    // let tup: (i32, f64, u8) = (666, 5.2, 1);
    // let tup = (666, 5.2, 1);
    // let (x, y, z) = tup;
    // println!("x= {x}, y = {y}, z = {z}")

    // let tup: (i32, f64, u8) = (666, 5.2, 1);
    // let first = tup.0;
    // let second = tup.1;
    // let third = tup.2;
    // println!("first={first};second={second};third={third}")

    // let arr: [i8; 5] = [1, 2, 3, 4, 5];
    // let a1 = arr[1];
    // println!("arr1={a1}");

    // let a = [1, 2, 3, 4, 5];
    // println!("Please enter an array index.");
    
    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");
    
    // let element = a[index];
    
    // println!("The value of the element at index {index} is: {element}");


    // another_fn()

    // let y = {
    //     let x = 3;
    //     x + 1
    // };
    // println!("The value of y is: {y}");

    // let x = five();
    // println!("x={x}")

    // let x = plus_one(5);
    // println!("x={x}");

    // let num = 3;
    // if  num < 5 {
    //     println!("true")
    // } else {
    //     println!("false")
    // }

    // let condition = true;
    // let num = if condition { 5 } else { 6 };
    // println!("num = {num}")

    // let condition = true;
    // let num = if condition { 5 } else { "six" };
    // println!("num = {num}")

    // loop {
    //     println!("again")
    // }

//     let mut counter =  0;
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter * 3;
//         }
//     };
//     println!("result={result}");

//    let a = 2.0;
//    let b = 1.1;
//    let num = a * b;
//     println!("num={num}");

    // let mut count = 0;
    // 'lable_first: loop {
    //     println!("count={count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'lable_first;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;

    // }


    // let mut num = 3;

    // while num != 0 {
    //     println!("num = {num}");
    //     num -= 1;
    // }
    // println!("end");

    // let arr = [1, 2, 3, 4, 5];
    // let mut index = 0;

    // while index < arr.len() {
    //     println!("a[index] = {}", arr[index]);
    //     index += 1;
    // }

    // let arr = [1, 2, 3, 4, 5];
    // for element in arr {
    //     println!("element = {element}");
    // }

    for num in (1..4).rev() {
        println!("num = {num}");
    }



}

// fn another_fn() {
//     println!("another_fn")
// }

// fn five() -> i8 {
//     5
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }


// ------------------第三章-------------------------


