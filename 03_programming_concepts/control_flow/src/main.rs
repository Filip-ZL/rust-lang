fn main() {

    if_expr();
    compounded_exprs();

    // inline if statement
    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of the number is: {number}");

    // loop_exemple();
    // returning_loop();
    // labelling_loops();
    // while_loops();
    // collection_listing();
    countdown();
}

fn if_expr(){
    let number = 7;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
}

fn compounded_exprs(){
    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3 or 2");
    }
}

fn loop_exemple(){

    loop {
        println!("Again!");
    }
}

// loop with return value
fn returning_loop(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10{
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn labelling_loops(){
    let mut count = 0;

    'counting_up: loop {
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9{
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn while_loops() {

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF");
}

// listing through collection
fn collection_listing() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }
}

// countdown
fn countdown() {
    for number in (1..4).rev() {
        println!("{number}!")
    }
    println!("LIFTOFF")
}