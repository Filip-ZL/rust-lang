fn main() {
    {
        let s1 = String::from("Hello");

        let len = calculate_length(&s1);

        println!("The length of '{s1}' is {len}.");
    }

    // References are immutable by default! 
    // if we want to change it, we need to specify!
    {
        let mut s1 = String::from("Hello");
        change(&mut s1);

        println!("{s1}");
    }

    // We cannot have more than one mutable 
    // reference to the same variable at time
    {

        let mut s = String::from("hello");

        let r1 = &mut s;
        // let r2 = &mut s;

        // println!("{}, {}", r1, r2);
    }

    // it's okay to have multiple imutable references
    // multiple users have access to the function 
    // (therefore, I don't expect its value to change!)
    {
        let mut s = String::from("hello");

        let s1 = &s;        // no problem
        let s2 = &s;        // no problem
        println!("{s1}, {s2}");
        // Variables s1, s2 not used after this point
        let s3 = &mut s;    

        // println!("{s1}, {s2} and {s3}"); // big problem
        println!("{s3}");
    }

    // dangling references!
    // rust makes sure no reference is dangling!
    {
        dangle();
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {

    s.push_str(", world!")
}

fn dangle() -> &String {
    let s = String::from("Hello");

    &s
}