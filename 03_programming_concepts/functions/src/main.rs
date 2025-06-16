fn main() {
    println!("Hello, world!");

    another_function(15);
    print_labeled_measurement(5, 'h');
    
    // Expression exemple (scope is an expression!)
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let z = plus_one(4);
    println!("Value of z is: {z}")

}

fn another_function(x: i32){
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value} {unit_label}");
}

// Function example with return value
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}