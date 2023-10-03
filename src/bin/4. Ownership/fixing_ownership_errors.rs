// 1. Returning a Reference to the Stack

// fn return_a_string() -> &String {
//     let s = String::from("hello");
//     &s
// }

// - here we return a reference to s
// but s goes out of scope and is dropped

// to fix this is to return the String directly

fn return_a_string() -> String {
    let s = String::from("hello");
    s
}

// we can also return a string literal

fn return_a_string2() -> &'static str {
    "hello"
}

// we can also use a reference-counted pointer
// this is a pointer that keeps track of the number of references to a value
// and only drops the value when the number of references is 0
// this will defer borrow-checking to runtime, making it slower

use std::rc::Rc;
fn return_a_string3() -> Rc<String> {
    let s = Rc::new(String::from("hello"));
    Rc::clone(&s)
}

// and we can have the caller provide a "slot" for us to write to

fn return_a_string4(output: &mut String) {
    output.replace_range(.., "hello");
}

// 2. Not Enough Permissions

// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
// }

// - push() requires W🟦 permission but name is an immutable reference

// to fix this we could make name a mutable reference
// but that is not a good solution! the user calling this function
// probably doesn't expect their variable to be mutated

fn stringify_name_with_title(name: &mut Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

// another option is to take ownership of the vector
// but this is also not a good solution!
// It is vary rare for a function to take ownership of heap-allocated data like Vec<T> and String
// It would make the name unusable after calling this function, because it would be dropped

fn stringify_name_with_title2(mut name: Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

// so the best solution is actually to use a reference but to clone the vector
// by cloning name we are allowed to mutate the local copy
// however clone() copies the entire vector, which is expensive

fn stringify_name_with_title3(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}

// we can add the suffix later and create a new string from the original vector

fn stringify_name_with_title4(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}

// 3. Aliasing and Mutating a Data Structure

fn main() {}
