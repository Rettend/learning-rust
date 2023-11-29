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
// It is very rare for a function to take ownership of heap-allocated data like Vec<T> and String
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

// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();

//     for s in src {
//         if s.len() > largest.len() {
//             dst.push(s.clone());
//         }
//     }
// }

// - we are trying to mutate dst while we are iterating over it
// the dst.push() call would invalidate the reference largest

// to fix this we could shorten the lifetime of largest by cloning it
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();

    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}

// but this could be expensive if the vector is large (it would clone the entire vector)

// another option is to perform length comparisons first and mutate the vector later
fn add_big_strings2(dst: &mut Vec<String>, src: &[String]) {
    let largest_len = dst.iter().map(|s| s.len()).max().unwrap();
    let to_add: Vec<String> = src
        .iter()
        .filter(|s| s.len() > largest_len)
        .cloned()
        .collect();

    dst.extend(to_add);
}

// but again, this is expensive because we are cloning the entire vector

// the best solution is to copy the length of the largest string
// since we don't need the contents of the string, only its length
fn add_big_strings3(dst: &mut Vec<String>, src: &[String]) {
    let largest_len = dst.iter().map(|s| s.len()).len();

    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}

fn main() {
    // 4. Copying vs. Moving Out of a Collection

    let v: Vec<i32> = vec![1, 2, 3];
    let n_ref: &i32 = &v[0];
    let n: i32 = *n_ref; // works because i32 implements Copy

    let v: Vec<String> = vec![String::from("Hello world!")];
    let s_ref: &String = &v[0];
    // let s: String = *s_ref; // doesn't work because String doesn't implement Copy

    // copying a String copies the pointer to the heap-allocated data
    // but we can't have two pointers to the same data, because when one of them is dropped
    // the other one would be invalid

    // i32 does not have this problem because it is stored on the stack
    // so what we can do is avoid taking ownership of the String
    // and just use an immutable reference
    let v: Vec<String> = vec![String::from("Hello world!")];
    let s_ref: &String = &v[0];
    println!("{s_ref}");

    // we can also clone the data if we want to take ownership
    let v: Vec<String> = vec![String::from("Hello world!")];
    let mut s: String = v[0].clone();
    s.push_str("!");
    println!("{s}");

    // or we can use Vec::remove() to move the String out of the vector
    let mut v: Vec<String> = vec![String::from("Hello world!")];
    let mut s: String = v.remove(0);
    s.push_str("!");
    println!("{s}");
    assert!(v.is_empty());

    // 5. Mutating Different Tuple Fields

    let mut name = (String::from("Ferris"), String::from("Bueller"));
    // - name: R🟧, W🟦, O🟥
    // - name.0: R🟧, W🟦, O🟥
    // - name.1: R🟧, W🟦, O🟥

    let first = &name.0;
    // - name: R🟧
    // - name.0: R🟧
    // - name.1: R🟧, W🟦, O🟥

    // mutating name.0 would invalidate first, but name.1 is fine
    name.1.push_str(", Jr."); // - name.1: R🟧, W🟦

    println!("{first} {}", name.1);
    // - first: R🟧, name.1: R🟧

    fn get_first(name: &(String, String)) -> &String {
        &name.0
        // - name: R🟧
    }

    let mut name = (String::from("Ferris"), String::from("Bueller"));

    let first = get_first(&name);
    // - first: R🟧
    // - name: R🟧
    // - name.0: R🟧
    // - name.1: R🟧, W🟦, O🟥

    // name.1.push_str(", Jr."); // - name.1: R🟧, W🟦
    println!("{first} {}", name.1);

    // this is still safe and does the same but with a function, but it no longer compiles
    // the problem is that Rust does not know that get_first() does not mutate name
    // so it assumes that name is borrowed for the entire lifetime of first

    // Mutating Different Array Elements

    // Lists don't work like tuples and Rust does not track which
    // elements are referenced by a reference, so all elements lose
    // their permissions until the reference is dropped

    let mut a = [0, 1, 2, 3];
    // - a: R🟧, W🟦, O🟥
    // - a[_]: R🟧, W🟦

    let x = &mut a[0];
    // - a: ❌
    // - a[_]: ❌

    *x += 1;
    // - a: R🟧, W🟦, O🟥
    // - a[_]: R🟧, W🟦

    println!("{a:?}");

    // let's say we want to read from an array index while mutating another index

    let mut a = [0, 1, 2, 3];

    let x = &mut a[0];

    let y = &a[1];
    // *x += *y; // error: cannot borrow `a[_]` as immutable because it is also borrowed as mutable

    // this doesn't work because Rust treats all elements of the array as borrowed
    // this code is safe but still rejected by the compiler
    // for cases like this, there are functions from the standard library that can help
    // like slice::split_first_mut()

    let mut a = [0, 1, 2, 3];
    let (x, rest) = a.split_first_mut().unwrap();
    let y = &rest[0];
    *x += *y;

    // but how is split_first_mut() implemented?
    // it uses unsafe code to create a temporary mutable reference to the array
    // and then it uses that reference to create a slice
    // this is safe because the slice is guaranteed to be valid for the lifetime of the reference
    // but it is not possible to implement this function in safe code because of the borrow checker
    // we could use unsafe to implement this function ourselves, but it is better to use the standard library

    let mut a = [0, 1, 2, 3];
    let x = &mut a[0] as *mut i32;
    let y = &a[1] as *const i32;
    unsafe {
        *x += *y;
    }
}
