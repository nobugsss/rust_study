fn main() {
    let school = School {
        name: String::from("xxx"),
        age: 100,
    };
    school.do_sth(|age| {
        println!("age: {:?}", age);
    })
}

struct School {
    name: String,
    age: usize,
}

impl School {
    fn do_sth<F>(&self, f: F)
    where
        F: FnOnce(usize),
    {
        println!("name: {:?}", self.name);
        f(self.age)
    }
}
