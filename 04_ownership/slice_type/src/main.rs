fn main() {
    println!("Hello, world!");

    let s = String::from("hello world");

    // let hello = &s[0..5];
    // let word = &s[6..11];

    let hello = first_word(&s);

    println!("first word of the string is: {hello}");
}


// error prone as it's not reflecting actual state!
fn first_word(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){

        if item == b' '{
            return &s[..i]
        }
    }
    &s[..]
}

