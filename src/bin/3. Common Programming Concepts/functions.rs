fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_parameter(5);
    print_labeled_measurement(5, 'h');

    // Statements and Expressions

    // Statements are instructions that perform some action and do not return a value.
    let y = 6;

    // Expressions evaluate to a resulting value.
    let _ = y + 1;

    // A new scope block created with curly brackets is an expression.
    let y = {
        let x = 3;
        x + 1 // Note the lack of a semicolon here. If we add a semicolon, this becomes a statement and does not return a value.
    };

    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

// Parameters

fn another_function() {
    println!("Another function.");
}

fn another_function_with_parameter(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("the measurement is: {value}{unit_label}");
}

// Funcitions with Return Values

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}