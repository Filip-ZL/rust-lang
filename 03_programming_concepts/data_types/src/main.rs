use std::io;

fn main() {
   
    // Scalar types exemples
    let guess: u32 = "42".parse().expect("Not a number");
    println!("Your guess is {guess}");

    let sum = 5 + 10;                                   // Addition
    println!("Sum: 5 + 10 = {sum}");

    let difference = 95.5 - 4.3;                        // Subtraction
    println!("Difference 95.5 - 4.3 = {difference}");

    let product = 4 * 30;                               // Multiplication
    println!("Product 4 * 30 = {product}");

    let quotient = 56.7 / 32.2;                         // Division
    let truncated = - 5 / 3;                            // Results in -1
    println!("Division 56.7 / 32.2 = {quotient}");
    println!("Truncated output -5 / 3 = {truncated}");

    let remainder = 43 % 5;                             // Remainder
    println!("Remainder 43 mod 5 = {remainder}");

    // Boolean dtypes
    let t = true;
    let f: bool = false;

    // Characters

    let c = 'z';
    let emoji = '😬';
    println!("Characters can be {c} | {emoji}");

    // Compound type exemples

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let tup = (500, 6.4, 1);
    let (x, y, z) = tup;                                 // Destructure tuple
    println!("Value of y is: {y}");

    let five_houndred = tup.0;                           // Accessing the elements of tuple

    // Arrays - 'stored in stack'
    // Fixed in size --> Unlike vectors that might differ in size
    let a = [1, 2, 3, 4, 5];

    // Good idea to use array --> Months are fixed length!
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    //     dtype length
    //       ↓   ↓
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    //     num  repeat --> this is same as a = [3, 3, 3, 3, 3]
    //       ↓  ↓
    let b = [3; 5];

    // Accessing array elements
    let first = a[0];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}
