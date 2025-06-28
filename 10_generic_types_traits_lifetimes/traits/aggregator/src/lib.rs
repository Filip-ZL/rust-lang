use std::fmt::{Debug, Display};
pub trait Summary {
    fn summarize_author(&self) -> String;
    // We can specify the trait default behavior or just signautre
    // signature would be: fn summarize(&self) -> String;
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

impl Summary for NewsArticle {
    // If we define summarize we override the default behavior
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline,self.author, self.location)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Using trait bounds to conditionally implement methods
pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T>{
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We define function that accepts any type that implements Summary trait!
// Shorter version of notify!
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }
// Longer version
// pub fn notify<T: Summary>(item: &T){
//     println!("Breaking news! {}", item.summarize());
// }

// This way we force the item1 and item2 to be the same type T
// pub fn notify<T: Summary,>(item1: &T, item2: &U){
//     println!("Breaking news! {}", item1.summarize());
//     println!("Breaking news! {}", item2.summarize());
// }

// this way we don't care if item1 and item2 are different type
pub fn notify(item1: &impl Summary, item2: &impl Summary){
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

// Specify multiple trait bounds
// pub fn notify(item1: &(impl Summary + Display)) {}
// pub fn notify<T: Summary + Display>(item1: &T, item2: &T){}

// in case of multiple trait bounds rust uses alternative syntax using where
// function below works same way as function commented below
// pub fn some_function<T: Summary + Display, U: Clone + Debug>(item1: &T, item2: &U){}
pub fn some_function<T, U>(item1: &T, item2: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug
{
    0
}

// Returning types that implements trait
fn return_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        repost: false,
    }
}