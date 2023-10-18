use rand::Rng;

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Patterns That Bind to Values

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents2(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    // Matching with Option<T>

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            None => None,
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    fn plus_one2(x: Option<i32>) -> i32 {
        match x {
            Some(i) => i + 1,
            None => 0,
        }
    }

    let five = Some(5);
    let six = plus_one2(five);
    let none = plus_one2(None);

    // Matches Are Exhaustive

    // fn plus_one3(x: Option<i32>) -> i32 {
    //     match x { // error: non-exhaustive patterns: `None` not covered
    //         Some(i) => i + 1,
    //     }
    // }

    // Catch-all Patterns and the _ Placeholder

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}

    let dice_roll = rand::thread_rng().gen_range(1..=6);

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // _ => reroll(),
        // _ => (),
    }

    // Matches and Ownership

    let opt: Option<String> = Some(String::from("Hello!"));
    // - opt: R🟧, O🟥
    // - opt@Some.0: R🟧, O🟥

    match opt { // - opt: R🟧
        Some(_) => println!("Some!"),
        // error!
        // Some(s) => println!("Some: {s}"), // - s: R🟧
        // - opt: ❌
        // - opt@Some.0: ❌
        None => println!("None!"),
    }

    println!("{:?}", opt); // - opt: R🟧

    match &opt { // - opt: R🟧
        // - opt: R🟧
        // - opt@Some: R🟧
        Some(s) => println!("Some: {s}"), // - s: R🟧
        // - opt: R🟧, O🟥
        // - opt@Some: R🟧, O🟥
        None => println!("None!"),
    }

    println!("{:?}", opt); // - opt: R🟧

}
