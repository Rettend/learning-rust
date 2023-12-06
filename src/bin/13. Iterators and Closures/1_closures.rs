use std::{thread, time::Duration, vec};

// Capturing the Environtment with Closures
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // Closure Type Inference and Annotation
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v4 = |x| x + 1;

    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // let n = example_closure(5); // error: mismatched types expected `String`, found integer rustc(E0308)

    // Capturing References or Moving Ownership
    // immutable borrow
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // mutable borrow
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // println!("Before calling closure: {:?}", list); // error: cannot borrow `list` as immutable because it is also borrowed as mutable
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    // move
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // Moving Captured Values Out of Closures and the Fn Traits
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:?}", list);
    println!("{:#?}", list);

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);

    // Closures Must Name Captured Lifetimes
    // fn make_a_cloner(s_ref: &str) -> impl Fn() -> String {
    //     move || s_ref.to_owned() // error! borrowed data cannot be stored outside of its closure
    // }

    fn make_a_cloner(s_ref: &str) -> impl Fn() -> String + '_ {
        move || s_ref.to_owned()
    }

    fn make_a_cloner2<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
        move || s_ref.to_owned()
    }

    let s = String::from("Hello");
    let cloner = make_a_cloner(&s);

    // drop(s); // error! cannot move out of `s` because it is borrowed

    
}
