// Defining a Trait
pub trait Summary {
    fn summarize(&self) -> String;
}

// Implementing a Trait on a Type
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // Default Implementations
    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    impl Summary for NewsArticle {}

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    pub struct Tweet2 {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    pub trait Summary2 {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    impl Summary2 for Tweet2 {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let tweet = Tweet2 {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // Traits as Parameters
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // Trait bound syntax
    pub fn notify2<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    // Specifying multiple trait bounds with the + syntax
    use std::fmt::{Debug, Display};

    pub fn notify3(item: &(impl Summary + Display)) {}

    pub fn notify4<T: Summary + Display>(item: &T) {}

    // Clearer trait bounds with where clauses
    fn some_function<
        T: Display + Clone,
        U: Clone + Debug,
        X: Display + Clone + Debug,
        Z: Clone + Debug,
    >(
        t: &T,
        u: &U,
    ) -> i32 {
        0
    }

    fn some_function2<T, U, X, Z>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
        X: Display + Clone + Debug,
        Z: Clone + Debug,
    {
        0
    }

    // Returning Types that Implement Traits
    fn returns_summarizable() -> impl Summary2 {
        Tweet2 {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }

    fn returns_summarizable2(switch: bool) -> impl Summary {
        if switch {
            NewsArticle {
                headline: String::from("Penguins win the Stanley Cup Championship!"),
                location: String::from("Pittsburgh, PA, USA"),
                author: String::from("Iceburgh"),
                content: String::from(
                    "The Pittsburgh Penguins once again are the best \
                     hockey team in the NHL.",
                ),
            }
        } else {
            // Tweet {}
            // error! `if` and `else` have incompatible types

            NewsArticle {
                headline: String::from("Penguins win the Stanley Cup Championship!"),
                location: String::from("Pittsburgh, PA, USA"),
                author: String::from("Iceburgh"),
                content: String::from(
                    "The Pittsburgh Penguins once again are the best \
                     hockey team in the NHL.",
                ),
            }
        }
    }

    // Using Trait Bounds to Conditionally Implement Methods
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x == self.y {
                println!("The members are equal");
            } else if self.x > self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

}
