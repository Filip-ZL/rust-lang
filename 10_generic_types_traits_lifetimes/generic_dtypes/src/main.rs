

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_int(&number_list);
    println!("The largest number is {result}");
    //using generics
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['a', 'c', 'd', 'f', 'p'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    //using generics
    let result = largest(&char_list);
    println!("The largest number is {result}");

    let coordinates = Point {x: 7, y: 8};
    let coordinates_2 = Point {x: 7.85, y: 8.21};
    
    println!("p.x = {}", coordinates.x());
    // Note: for Point struct T has to be same over the whole structure
    // Thus, code below would not compile
    // let coor = Point {x:6, y:7.5}; // TRY ME

    // We can specify multiple generics like structure User (check)
    // Imagine we have a company that uses old system for user names
    // in form of integers but new system is more freed
    // same goes for favorite numbers... one can like pi other can like integers, etc.
    let user_1 = User {uname: 21544, favorite_number: 3.14};
    let user_2 = User {uname: "Peter", favorite_number: 4};

}

// Motivation --> We have 2 functions below 
// providing the same task = return largest value from slice
// (largest_char, largest_i32)
fn largest_char(list: &[char]) -> &char {
    
    let mut largest = &list[0];
    for c in list {
        if c > largest{
            largest = c;
        }
    }

    largest
}

fn largest_int(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for i in list {
        if i > largest {
            largest = i
        }
    }

    largest
}

// We can cobine 2 functions above using generics

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

// we can use generics for structs as well
struct Point<T> {
    x: T,
    y: T,
}

impl <T> Point<T>{
    fn x(&self) -> &T{
        &self.x
    }
}
// we can define methods only on functions with specific type
impl Point<f32>{
    fn distance_from_origin(&self) -> f32 {
       (&self.x.powi(2) + &self.y.powi(2)).sqrt()
    }
}

struct User<T, U>{
    uname: T,
    favorite_number: U,
}

// enum follows similar rules as struct
enum Car<T> {
    Drive(T),
    Stop,
}

// Generic type params in struct definition aren't always 
//the same as those you use in that same strust's method signatures.
struct Coordinates<X1, Y1>{
    x: X1,
    y: Y1,
}

impl<X1, Y1> Coordinates<X1,Y1>{

    fn mixup<X2, Y2>(self, other: Coordinates<X2, Y2>) -> Coordinates<X1, Y2>{
        Coordinates {
            x: self.x,
            y: other.y,
        }
    }
}