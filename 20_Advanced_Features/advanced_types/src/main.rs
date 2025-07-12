fn main() {
    // Using the Newtype Pattern for Type Safety and Abstraction
    {
        // Creating Type synonymous with type Aliases
        type Kilometers = i32;

        let x: i32 = 5;
        let y: Kilometers = 5;

        println!("x + y = {}", x + y);

        // Actual usage -> for example with lenghty type like this
        type Thunk = Box<dyn Fn() + Send + 'static>;
        let f: Thunk  = Box::new(|| println!("hi"));

        fn takes_long_type(f: Thunk) {
            // --snip--
        }

        fn returns_long_type() -> Thunk {
            // --snip--
        }

    }
}

// Type thatnever returns with `!`
fn bar() -> ! {
    // --snip--
}

// other examples of never types: panic!, continue, loop (without break)