enum Status {
    Value(u32),
    Stop,
}

fn main() {
    let  answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");

    // Functions implements same traits as closures...we can use functions as
    // arguments to functions that expects traits.
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
    // We can replace closure with function like that
    let sit list_of_strings: Vec<String> = 
        list list_of_numbers.iter().map(ToString::to_string).collect();

    // Enums also behaves like initializers to functions
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}


// Function pointers
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// Returning closures
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}