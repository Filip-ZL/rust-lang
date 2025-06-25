fn main() {

    // String init
    {
        let mut s = String::new();

        let data = "initial contents";

        let s = data.to_string();

        // method work on strings directly
        let s = "initial contents".to_string();

        // we can use function String::from --> known from before
        let s = String::from("initial contents");
    }

    // update a string
    {
        // Appending to string
        {
            let mut s = String::from("foo");
            s.push_str("bar");

            let s2 = "bar";
            s.push_str(s2); // --> push_str takes string slice as input
            println!("s2 is {s2}");

            let mut s3 = String::from("Hello");
            let s4 = String::from(", World!");

            s3.push_str(&s4); // --> again we need to pass string slice as input!
            println!("{s3}");

            // if we want to append one character we might use push method!
            let mut s = String::from("lo");
            s.push('l');
        }
        // Concatenation with the + Operator or the format! Macro
        {
            let s1 = String::from("Hello, ");
            let s2 = String::from("World!");
            let s3 = s1 + &s2;   // s1 was moved here and cannot be used!
            println!("CAT: {s3}");

            let s1 = String::from("tic");
            let s2 = String::from("tac");
            let s3 = String::from("toe");

            // you might use format! macro in case of 
            // multiple concatenation
            let s = s1 + "-" + &s2 + "-" + &s3;

            let s1 = String::from("tic");
            let s2 = String::from("tac");
            let s3 = String::from("toe");

            // this will do same as above! Its more readable!
            // format! macro takes references (similar to println! macro)
            // Thus, we can use the s1, s2, s3 vars afterwards!
            let s = format!("{s1}-{s2}-{s3}");
        }

        // Indexing
        {
            let s1 = String::from("Hi");
            // let h =  s1[0]; // this will result in compile error

            // separate string into chars 'H' and 'i' in this case
            for c in s1.chars() {
                println!("{c}");
            }

            // Separate string into bytes
            for b in s1.bytes(){
                println!("{b}");
            }
        }


    }
}
