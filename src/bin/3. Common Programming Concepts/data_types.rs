use std::io;

fn main() {
    // -- Scalar Types --

    // Integer Types

    // type annotation needed - rustc (E0282)
    // let guess = "42".parse().expect("Not a number!");

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("guess = {guess}");

    // Floating-Point Types

    let x = 2.0; // f64 - double precision
    let y: f32 = 3.0; // f32 - single precision

    println!("x = {x}, y = {y}");

    // Numeric Operations

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // -1

    let remainder = 43 % 5;

    println!("sum = {sum}\ndifference = {difference}\nproduct = {product}\nquotient = {quotient}\ntruncated = {truncated}\nremainder = {remainder}");

    // The Boolean Type

    let t = true;
    let f: bool = false;

    println!("t = {t}, f = {f}");

    // The Character Type

    let c = 'ű';
    let z: char = 'Á';
    let heart_eyed_cat = '😻';

    println!("c = {c}, z = {z}, heart_eyed_cat = {heart_eyed_cat}");

    // -- Compound Types --

    // The Tuple Type

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("x = {x}, y = {y}, z = {z}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("five_hundred = {five_hundred}\nsix_point_four = {six_point_four}\none = {one}");

    // The Array Type

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("first element = {}", a[0]);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("last element = {}", months.last().unwrap());

    let a = [3; 5]; // [3, 3, 3, 3, 3]

    println!("sum of a = {}", a.iter().sum::<i32>());

    array_indexing_game();
}

const READ_LINE_ERROR: &str = "Failed to read line.";
const PARSE_NUMBER_ERROR: &str = "Please type a number!";

fn array_indexing_game() {
    println!("-- [ Array Indexing Game ] --");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect(READ_LINE_ERROR);

    let index: usize = index.trim().parse().expect(PARSE_NUMBER_ERROR);

    // This will panic if the index is out of bounds.
    // let element = a[index];

    // Instead, we can use the `get` method to return `None` if the index is out of bounds.
    let element = a.get(index);

    println!(
        "The value of the element at index {} is: {}",
        index,
        element.unwrap_or(&-1)
    );
}
