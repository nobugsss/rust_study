fn main() {
    // let mut user1 = User {
    //     active: true,
    //     username: String::from("boykaaaa"),
    //     email: String::from("tlxc1224@gmail.com"),
    //     sign_in_count: 1,
    // };

    // user1.email = String::from("tlxc1224@163.com");

    // let user2 = User {
    //     email: String::from("xxx@gmail.com"),
    //    ..user1,
    // }

    // let black = Color(0, 0, 0);
    // let origin = Point(0, 0, 0);

    // let width1 = 30;
    // let height1 = 30;
    // println!("The area of the rectangle is {} square pixels.", area(width1, height1));

    // let rect1 = (30, 50);
    // println!("The area of the rectangle is {} square pixels.", area(rect1));
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1={:?}", rect1);
    println!("The area of the rectangle is {} square pixels.", area(&rect1));


}

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn build_user(username: String, email: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensios: (u32, u32)) -> u32 {
//     dimensios.0 * dimensios.1
// }

 #[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
