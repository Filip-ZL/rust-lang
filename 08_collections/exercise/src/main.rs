use std::collections::HashMap;
use std::io;
use colored::Colorize;

fn main() {
    // Excercise 1:
    {
        let mut integer_vector= vec![0, 4, 8, 5, 3, 3, 7, 9, 11, 9, 6, 3, 9];
        let (x, y) = get_vector_stats(&mut integer_vector);
        println!("Median is {x} and most occured number is {y}");
    }

    // Exercise 2:
    {
        let word = String::from("AHello");

        let pig = pig_latin(&word);
        println!("{pig}")
    }

    // Exercise 3:
    {
        company_structure();
    }
}

// Function implements exercise 1
fn get_vector_stats(v: &mut Vec<i32>) -> (f64, i32){
    v.sort();
    let median: f64; 
    if v.len() % 2 == 0{
        median = v[v.len() / 2] as f64;
    } else {
        let lower = v[v.len() / 2];
        let higher = v[v.len() / 2 + 1];
        median = (higher + lower) as f64 / 2.0;
    }
    let mut occurance = HashMap::new();

    for val in v{
        *occurance.entry(*val).or_insert(0) += 1;
    }

    let mut most_occured = 0;
    let max_val = match occurance.values().max(){
        Some(v) => &v,
        None => &0,
    };
    for (key, value) in occurance.iter(){
        if value == max_val {
            most_occured = *key;
        }
    }
   
    (median, most_occured)
}

// Function implements exercise 2
fn pig_latin(text: &str) -> String{

    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    
    for c in text.chars(){
        if vowels.contains(&c.to_ascii_lowercase()){
            return format!("{text}-hay");
        } else {
            let residual = String::from(text).split_off(1);
            return format!("{residual}-{c}ay")
        }
    }

    String::new()
    // let mut text_chars = String::from(text).chars();
}

// Function implements exercise 3
fn company_structure(){
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut user_feedback = String::new();
        println!("Please select from following options \n
                  1) Assign User \n
                  2) Add department \n
                  3) List Department");
        match io::stdin()
                .read_line(&mut user_feedback) {
                    Ok(t) => t,
                    Err(_) => continue,
                };
        let option: u8 = match user_feedback.trim().parse() {
            Ok(t) => t,
            Err(_) => {
                println!("{}", "Invalid Input. Try again!".red());
                continue;
            },
        };
        match option {
            1 => assignuser(&mut departments),
            2 => adddepartment(&mut departments),
            3 => listdepartment(&departments),
            _ => {
                println!("{}", "Invalid Input. Try again!".red());
                continue; 
            }
        }
    }
}

fn assignuser(departments: &mut HashMap<String, Vec<String>>) {

    'main: loop {
        let mut user_feedback = String::new();
        println!("Please select department");
        if departments.keys().len() == 0{
            println!("{}", "No departments available. Please create some before user assignment!".red());
            break;
        }
        for (i, department) in departments.keys().into_iter().enumerate(){
            println!("{i}) {department}");
        }
        match io::stdin()
                .read_line(&mut user_feedback) {
                    Ok(t) => t,
                    Err(_) => continue,
        };
        let option: usize = match user_feedback.trim().parse() {
            Ok(t) => t,
            Err(_) => {
                println!("{}", "Invalid Department. Try again".red());
                continue;
            },  
        };
        let mut name = String::new();
        // io::stdin().read_line(&mut name).trim();
        println!("Enter new username:");
        match io::stdin()
                .read_line(&mut name) {
                    Ok(t) => t,
                    Err(_) => continue,
        };
        for (i, (_, value)) in departments.iter_mut().enumerate(){
            if i == option{
                value.push(name.trim().to_string());
                break;
            }
        }
        
        

        println!("{}", "User added succesfully".green());
        break 'main;
    }
}
fn adddepartment(departments: &mut HashMap<String, Vec<String>>) {
    loop {
        let mut user_feedback = String::new();
        println!("Please provide department name:");
        match io::stdin()
                .read_line(&mut user_feedback) {
                    Ok(t) => t,
                    Err(_) => continue,
        };
        departments.entry(user_feedback.trim().to_string()).or_insert(Vec::new());
        println!("{}", "Department added succesfully".green());
        break;
    }
}
fn listdepartment(departments: &HashMap<String, Vec<String>>) {

    loop {
        let mut user_feedback = String::new();
        println!("Please select department: ");
        for (i, department) in departments.keys().into_iter().enumerate(){
            println!("{i}) {department}");
        }
        match io::stdin()
                .read_line(&mut user_feedback) {
                    Ok(t) => t,
                    Err(_) => continue,
        };
        let option: usize = match user_feedback.trim().parse(){
            Ok(t) => t,
            Err(_) => {
                println!("{}", "Invalid Department. Try again".red());
                continue;
            },  
        };
        let mut deps = departments.clone();
        for (i, (key, value)) in deps.iter_mut().enumerate(){
            
            if i == option{
                value.sort();
                println!("{}: {:?}", key, value);
            }
        };
        break;
    }
}