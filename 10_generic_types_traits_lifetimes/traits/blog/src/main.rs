use aggregator::{self, SocialPost, NewsArticle, Summary};

fn main(){
    let post = SocialPost {
        username: String::from("Filip"),
        content: String::from("Hello there, this is my account where I'll be sharing interesting content about rust"),
        reply: false,
        repost: false,
    };

    println!("{}", post.summarize());

    let article = NewsArticle {
        headline: String::from("Rust basics"),
        location: String::from("Czechia"),
        author: String::from("Filip"),
        content: String::from("Learn more about rust traits! ..."),
    };
    println!("{}", article.summarize());
    aggregator::notify(&post, &article);
}