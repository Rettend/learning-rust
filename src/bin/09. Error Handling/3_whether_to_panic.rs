use std::net::IpAddr;
use std::cmp::Ordering;

fn main() {
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should parse");

    let guess = "42".parse::<i32>();
    let secret_number = 42;

    loop {
        let guess: i32 = match guess {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            Guess::validate_value(value);
            Guess { value }
        }

        pub fn get(&self) -> i32 {
            self.value
        }

        pub fn set(&mut self, value: i32) {
            Guess::validate_value(value);
            self.value = value;
        }

        fn validate_value(value: i32) {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
        }
    }

    let mut guess = Guess::new(12);
    guess.set(13);

    println!("Guess value: {}", guess.get());
}
