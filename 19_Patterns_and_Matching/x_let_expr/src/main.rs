fn main() {
    // Conditional if let Expressions
    {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {color} as the background");
        } else if is_tuesday{
            println!("Tuesday is a green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using oragne as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }

    // Using while let conditional loops
    {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            for val in [1, 2, 3]{
                tx.send(val).unwrap();
            }
        });
        // This loop will go on as as long as thread is spawning values!
        while let Ok(value) = rx.recv() {
            println!("{value}");
        }
    }

    // Using for loops
    {
        let v = vec!['a', 'b', 'c'];
        //    pattern is following the for keyword
        //       ↓ 
        for (index, value) in v.iter().enumerate() {
            println!("{value} is at index {index}");
        }
    }

    // let statements
    {
        // let statement looks like This
        // let PATTERN = EXPRESSION;
   // pattern   expression
        //  ↓   ↓
        let x = 5;

    }

    // function parameters
    {
        // basic
        //  pattern
        //     ↓
        fn foo(x: i32) {
            // code goes here
        }

        // that means we can match a tuple in a function's arguments to the
        // pattern

        fn print_coordinates(&(x, y): &(i32, i32)){
            println!("Current location: ({x}, {y})");
        }

        let point = (3, 5);
        print_coordinates(&point);

    }
}
