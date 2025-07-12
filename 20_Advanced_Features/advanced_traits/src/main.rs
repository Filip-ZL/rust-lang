use std::ops::Add;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Types that share variable with traits they implement
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }

}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Supertraits - traits that deppend on other traits!
// in this case Display is supertraitfor OutlinePrint
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
} 

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
} 

impl OutlinePrint for Point {}

// Newtype pattern (walk around the orphan rule)
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(",  "))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3},
        Point { x: 3, y: 3}
    );
    
    // Function  default prints name implemented under Dog struct
    // means - this prints 'A baby dog is called Spot
    println!("A baby dog is called a  {}", Dog::baby_name());

    // if we want to use function under Animal trait we need to use qualified
    // syntax (<Typeas Trait>::function(receiver_if_method, next_arg, ...))
    println!("A baby dog is called a  {}", <Dog as Animal>::baby_name());

    let a = Point { x: 1, y: 3 };

    a.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}