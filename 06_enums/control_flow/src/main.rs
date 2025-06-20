enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool{
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // --snip--
        }
    }
}

fn main() {
    
    let mut coin = Coin::Penny;

    let cents: u8 = value_in_cents(coin);

    println!("You've got {cents} cents!");

    let another_coin = Coin::Quarter(UsState::Alabama);
    let cents = value_in_cents(another_coin);

    // matching with Option
    let five: Option<i32> = Some(5);
    let six = plus_one(five);

    // catch all eample
    let dice = 9;
    match dice {
        7 => add_fancy_hat(),
        3 => remove_fancy_hat(),
        other => move_player(other),
    }

    // We have no intention to use the variable 
    // --> instead of other we go with _
    match dice {
        7 => add_fancy_hat(),
        3 => remove_fancy_hat(),
        _ => reroll(),
    }

    // do nothing in _
        match dice {
        7 => add_fancy_hat(),
        3 => remove_fancy_hat(),
        _ => (),
    }

    // if let statement
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    // this is the same as
    match config_max {
        Some(max) =>{
             println!("The maximum is configured to be {max}");
        }
        _ => () 
    }

    // we can use else branch with if let
    coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {state:?}")
    }
    else {
        count += 1;
    }
    describe_state_quarter(coin);

}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}

fn plus_one(num: Option<i32>) -> Option<i32>{

    match num {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {

    // we could use let..else statement
    // if let Coin::Quarter(state) = coin {
    //     if state.existed_in(1900) {
    //         Some(format!("{state:?} is pretty old, for America!"))
    //     }
    //     else {
    //         Some(format!("{state:?} is relatively new"))
    //     }
    // } else {
    //     None
    // }
    let Coin::Quarter(state) = coin else {
        return None
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new"))
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num: u8) {}
fn reroll(){}