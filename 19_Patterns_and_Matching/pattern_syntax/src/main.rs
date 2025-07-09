struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

fn main() {
    // Matching Literals
    {
    // Matching patterns against literals directly
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    // code above prints 'one' because our value is 1.
    }

    // Matching Named Variables
    {
        // named variables = irrefutable patterns that match any value.
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {y}"),
            _ => println!("Default case, x = {x:?}"),
        }

        println!("at the end: x = {x:?}, y = {y}");
    }

    // Multiple patterns
    {
        let x = 1;
        match x {
        //   or oparator --> if x is 1 or 2 expression behind got executed
        //    ↓
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    // Matching Ranges of Values with  ..=
    {
        let x = 5;
        match x {
            1..=5 => println!("One through five"),
            _ => println!("something else"),
        }

        // using char values
        let x = 'c';
        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }

    /// Destructuring to Break Apart Values
    // Dstructuring structs
    {
        let p = Point {x: 0, y: 7};

        let Point {x: a, y: b} = p;
        assert_eq!(0, a);
        assert_eq!(7, b);

        // same outcome, different approach
        let Point{x, y} = p;
        assert_eq!(0, x);
        assert_eq!(7, b);

        match p {
            Point { x, y: 0 } => println!("On the x axis at {x}"),
            Point { x: 0, y } => println!("On the y axis at {y}"),
            Point { x, y } => {
                println!("On neither axis: ({x}, {y})");
            }
        }
    }

    // Deconstructiong enums
    {
        let msg = Message::ChangeColor(0, 160, 255);
        // let msg = Message::Write(String::from("Hello world"));

        match msg {
            Message::Quit => {
                println!("The Quit vairant has no data to destructure");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and the y direction {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change color to red {r}, green {g} and blue {b}");
            }
        }
    }

    // Destructuring Nestd Structs and Enums
    {
        enum Message {
            Quit,
            Move {x: i32, y: i32},
            Write(String),
            ChangeColor(Color)
        }

        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change color to hue {h}, saturation {s}, value {v}");
            }
            _ => (),
        }
    }

    // Destructuring Structs and Tuples
    {
        let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10});
    }

    // Ignoring Values in pattern
    {
        // An Entire value with _
        fn foo(_: i32, y: i32) {
            println!("This code only uses y parameter:  {y}");
        }

        foo(3, 4);                                                              // This code will completely ignor the 3
                                                                                // and only uses 4 bind as y value
        // Parts of a Value with a Nested _
        let mut setting_value = Some(5);
        let new_seeting_value = Some(10);

        match (setting_value, new_seeting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_seeting_value;
            }
        }

        println!("setting is {setting_value:?}");

        // ignoring inmultiple places within one pattern to ignore particular
        // values
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {first}, {third}, {fifth}");
            }
        }
    }

    // Remaining Parts of a Value with ..
    {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }
        let origin = Point { x: 0, y: 0, z: 0};

        match origin {
            Point { x, .. }  => println!("x is {x}"),
        }

        // using .. with tuple
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {first}, {last}");
            }
        }
    }

    // Extra Conditioonals with Match Guards
    {
        let num = Some(4);
        // we can create more complex matching using match guard,
        // however, compiler won't not check for exhaustiveness.
        match num {
            Some(x) if x % 2 == 0 => println!("The number {x} is even"),
            Some(x) => println!("The number {x} is odd"),
            None => (),
        }

        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matchd, n = {n}"),
            _ => println!("Default case, x = {x:?}"),
        }

        println!("at the end: x = {x:?}, y = {y}")
    }

    // @ Bindings
    {
        enum Message {
            Hello { id: i32 },
        }
        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => println!("found an id in range: {id_variable}"),
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range");
            }
            Message::Hello { id } => println!("Found some other id: {id}"),
        }
    }

}
