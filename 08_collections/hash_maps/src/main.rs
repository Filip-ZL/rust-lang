use std::collections::HashMap;

fn main() {
    
    // Basics
    {    
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);

        println!("Team {team_name}'s score: {score}");

        for (key, value) in &scores {
            println!("{key}: {value}");
        }

        // ownership --> HashMap implements Copy trait for types like i32, ...
        // owned values like string will be moved!

        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // field_name and field_value are invalid at this point, try using them
        // and see what compiler error you get!
        // we can insert references into HashMap but they need to stay valid!
    }

    // Updating HashMap
    {
        let mut scores = HashMap::new();
        
        // keys must be unique --> the original value will be replaced
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);
        println!("{scores:?}");

        // Adding key and value only if key isn't present
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{scores:?}");

        // Updating a value based on the old value
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace(){
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{map:?}");

    }
}
