fn main() {

    let deg_c: f64 = 32.0;
    let deg_f: f64 = convert_to_fahrenheit(deg_c);

    println!("{deg_c} °C is {deg_f} °C.");

    // nth fibonaci number
    let n: usize = 10;
    let fib_n: usize = nth_fibonaci_number(n);

    println!("{n}th number of Fibonacci sequence is: {fib_n}");
    
    lyrics_generator();
}


fn convert_to_fahrenheit(deg_c: f64) -> f64{

    (deg_c * 9.0 / 5.0) + 32.0
}

fn nth_fibonaci_number(n: usize) -> usize{

    let mut previous: usize = 0;
    let mut _previous: usize = 0;
    let mut current: usize = 0;

    for element in (0..n){
        if (n == 0){
           break; 
        } else if (element < 1) {
            current = 1;
            previous = current;
        } else {
            current = previous + _previous;
            _previous = previous;
            previous = current;
        }
    }

    current
}

fn lyrics_generator(){

    let day_numbers = ["first", "second", "third", "fourth", "fifth", "sixth",
                       "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    
    let present_counter = ["A", "Two", "Three", "Four", "Five", "Six", 
                           "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve"];
    let presents = ["partridge in a pear tree", "turtle doves", "French hens",
                    "calling birds", "rings", "geese a-laying", "swans a swimming",
                    "maids a-milking", "ladies dancing", "lords a-leaping", 
                    "pipers piping", "drummers drumming"
                    ];

    let mut counter: usize = 1;
    for day_number in day_numbers{
        println!("On the {day_number} day of Chrismas,");
        println!("My true love sent to me");
        for count in (0..counter).rev(){
            let current_present = presents[count];
            let present_count = present_counter[count];
            let sentence_end = if count == 0 {'.'} else {','};
            if (day_number != "first") && (count == 0){
                println!("And a {current_present}{sentence_end}");
                continue;
            }
            println!("{present_count} {current_present}{sentence_end}");
        }
        println!("\n");
        counter += 1;

    }
}