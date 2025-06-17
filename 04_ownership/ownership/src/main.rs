fn main() {
    
    {   // variable s not valid here as it's not declared
        let s = "Hello!"; // s is valid from this point forward
    }   // this scope is now over and s is no longer valid

    {
        let mut s = String::from("Hello"); // initialize string from string literal!

        // s is mutable
        s.push_str(", world!");
        println!("{s}"); // Prints "Hello, world!"
    }       // variable s is out of scope here, is no longer valid!

    {
        let x = 5;
        let y = x;          // Integer is stored at stack --> makes copy of x, store both variables

        let s1 = String::from("Hello");   
        let s2 = s1   ;                  // String is stored on heap --> s1 is dropped!
    }

    {
        let mut s = String::from("Hello");
        s = String::from("ahoj");           // previous value is dropped as well!

        println!("{s}, world!");
    }

    // using copy
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();              // makes a copy of s1 and allocate its place in heap!
                                          // use with caution! may be memory expensive

        println!("s1 = {s1}, s2 = {s2}");
    }

    // ownership and functions
    {
        let s = String::from("Hello");

        takes_ownership(s);                     // s's value moves into the function...
                                                // ... and so is no longer valid here!
        
        let x = 5;                              // x comes into scope
        makes_copy(x);                          // because i32 implements the Copy trait,
                                                // x does NOT move into the function,
                                                // so it's okay to use x afterward!
        println!("{}", x);
    }   // Here, x goes out of scope, then s. But because s's value was moved, nothing
        // special happens.
        
    {
        let s1 = gives_ownership();             // gives_ownership moves its return
                                                // value into s1

        let s2 = String::from("hello");         // s2 comes into scope
        let s3 = takes_and_gives_back(s2);      // s2 is moved into
                                                // takes_and_gives_back, which also
                                                // moves its return value into s3
    }  // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.
}

fn takes_ownership(some_string: String) {       // Some string comes into the scope
    println!("{some_string}");
}   // Here, some_string goes out of scope and 'drop' is called. The backing
    // memory is freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.


fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}