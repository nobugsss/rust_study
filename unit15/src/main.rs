use std::mem::drop;
// #[derive(Debug)]
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};

fn main() {
    // enum List {
    //     Cons(i32, List),
    //     Nil,
    // }
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // println!("list: {:?}", list);

    // 15.2. 使用 Deref Trait 将智能指针当作常规引用处理

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // 必须使用 *y 来追踪引用所指向的值（也就是 解引用），这样编译器就可以比较实际的值了
    assert_eq!(5, *y);

    // 像引用一样使用 Box<T>
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // 自定义智能指针
    // let x = 5;
    // let y = MyBox::new(x);

    // assert_eq!(5, x);
    // assert_eq!(5, *y);
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("y: {:?}", *y);

    // 函数和方法的隐式 Deref 强制转换
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // 15.3. 使用 Drop Trait 运行清理代码
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    // 15.4. Rc<T> 引用计数智能指针
    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use std::rc::Rc;
    use List::{Cons, Nil};
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // let b = Cons(3, Rc::clone(&a));
    // let c = Cons(4, Rc::clone(&a));

    // 打印出引用计数
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// enum List {
//     Cons(i32, List),
//     Nil,
// }

// 自定义智能指针
use std::ops::Deref;
#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

// 15.3. 使用 Drop Trait 运行清理代码
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
