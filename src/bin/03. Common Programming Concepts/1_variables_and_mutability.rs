// Constants

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("{THREE_HOURS_IN_SECONDS}");

    // Mutability

    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // cannot mutate immutable variable `x` - rust-analyzer (E0384)

    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    // Shadowing

    let s = 5;

    let s = s + 1;

    {
        let s = s * 2;
        println!("The value of s in the inner scope is: {s}");
    }

    println!("The value of s is: {s}");

    // reassigning a variable's type with shadowing
    let spaces = "  ";
    let spaces = spaces.len();

    // mut doesn't allow reassigning a variable's type
    // let mut spaces = "  ";
    // spaces = spaces.len(); // expected `&str`, found `usize` - rust-analyzer (E0308)

    println!("spaces = {spaces}");
}
