fn main() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        // this respects the borrow-checkers rule... will compile
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
    // borrow parameter does not live long enough.
    // println!("The longest string is {result}");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

}

// parameter 'a fits lifetime parameter that 
//is long as smaller of the lifetimes x, y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len() {x} else {y}
}

// lifetimes with struct
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, annoucement: &str) -> &str {
        println!("Attention please: {annoucement}");
        self.part
    }
}

// Generic types, Trait Bounds and Lifetimes Together
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}