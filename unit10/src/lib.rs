// 10.2.Trait：定义共同行为
pub trait Summary {
    // fn summarize(&self) -> String;

    // Summary trait 的 summarize 方法指定一个默认的字符串值
    // fn summarize(&self) -> String {
    //     String::from("(Read more...)")
    // }

    // 默认实现允许调用相同 trait 中的其他方法，哪怕这些方法没有默认实现。如此，trait 可以提供很多有用的功能而只需要实现指定一小部分内容
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// 不再直接为 NewsArticle 定义 summarize 方法，但是提供了一个默认实现并且指定 NewsArticle 实现 Summary trait
// impl Summary for NewsArticle {}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("ccccc{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
