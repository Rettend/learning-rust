// Ownership vs Garbage Collection

type Document = Vec<String>;

// consumes ownership of the vector
// this way words will be dropped when the function ends
fn new_document(words: Vec<String>) -> Document {
    words
}

// borrows a mutable reference to the vector
fn add_word(this: &mut Document, word: String) {
    this.push(word);
}

// borrows an immutable reference to the vector
fn get_words(this: &Document) -> &[String] {
    this.as_slice()
}

fn main() {
    let words = vec!["hello".to_string()];
    let d = new_document(words);

    let words_copy = get_words(&d).to_vec();
    let mut d2 = new_document(words_copy);
    add_word(&mut d2, "world".to_string());

    // modifying d2 does not affect d
    assert!(!get_words(&d).contains(&"world".into()));

    // The Concepts of Ownership

    // - Rust allocates local variables when a function is called and frees them when the function exits
    // - Local variables can hold either data or pointers
    // - Pointers can be created either through:
    //   - boxes: pointers owning data on the heap
    //   - references: pointers borrowing data from another variable (not owning)

    fn inner(x: &mut i32) {
        let another_num = 1;
        let a_stack_ref = &another_num;

        let a_box = Box::new(2);
        let a_box_stack_ref = &a_box;
        let a_box_heap_ref = &*a_box;

        *x += 5;
    }

    let mut a_num = 0;
    inner(&mut a_num);

    // Slices

    let s = String::from("012345678");
    let s_slice = &s[2..5];

    // index starts at 0
    // start index is inclusive: 2 is included
    // end index is exclusive: 5 is not included
    assert_eq!(s_slice, "234");

    // Ownership at Compile-time

    // Rust tracks three permissions for each variable: R🟧, W🟦, O🟥

    // variables not marked with mut are immutable by default

    let n = 0;
    // - n: R🟧, O🟥

    // n += 1; // - n: R🟧, W🟦

    // A variable's permissions can be changed if it is moved or borrowed

    // A move of a variable with a non-copyable type (like String or Box<T>) requires the
    // R🟧 and O🟥 permissions, and the move eliminates all permissions of the source variable

    let s = String::from("Hello world");
    // - s: R🟧, O🟥

    consume_a_string(s); // - s: R🟧, O🟥
                         // - s: ❌

    // println!("{s}"); // - s: R🟧

    fn consume_a_string(_s: String) {
        // om nom nom
    }

    // Borrowing a variable (creating a reference to it) temporarily removes some permissions
    // An immutable reference prevents the data from being mutated or moved (W🟦, O🟥)

    let mut s = String::from("Hello");
    // - s: R🟧, W🟦, O🟥

    let s_ref = &s;
    // - s: R🟧
    // - s_ref: R🟧, O🟥
    // - *s_ref: R🟧

    println!("{s_ref}"); // - s_ref: R🟧
                         // - s: R🟧, W🟦, O🟥
                         // - s_ref: ❌
                         // - *s_ref: ❌

    println!("{s}"); // - s: R🟧,
                     // - s_ref: ❌

    // Mutating an immutable reference is not allowed

    let mut s = String::from("Hello");

    let s_ref = &s;
    // - *s_ref: R🟧

    // s_ref.push_str(", world!"); // - s_ref: R🟧, W🟦
    println!("{s}");

    // Mutating the immutable reference's source variable is not allowed

    let mut s = String::from("Hello");
    // - s: R🟧, W🟦, O🟥

    let s_ref = &s;
    // - s: R🟧

    // s.push_str(", world!"); // - s: R🟧, W🟦
    println!("{s_ref}");

    // And moving the data out of the reference is not ok either

    let mut s = String::from("Hello");

    let s_ref = &s;
    // - *s_ref: R🟧

    // let s2 = *s_ref; // - s_ref: R🟧, O🟥
    println!("{s}");

    // A mutable borrow creates a mutable reference, which prevents the data from being read, written, or moved
    // Mutating a reference is ok:

    let mut s = String::from("Hello");
    // - s: R🟧, W🟦, O🟥

    let s_ref = &mut s; // - s: R🟧, W🟦
                        // - s: ❌
                        // - s_ref: R🟧, O🟥
                        // - *s_ref: R🟧, W🟦

    s_ref.push_str(", world!"); // - s_ref: R🟧, W🟦
                                // - s: R🟧, W🟦, O🟥
                                // - s_ref: ❌
                                // - *s_ref: ❌

    println!("{s}");
    // - s: ❌

    // But accessing the mutably borrowed data is not ok:

    let mut s = String::from("Hello");

    let s_ref = &mut s;
    // - s: ❌

    // println!("{s}"); // - s: R🟧
    s_ref.push_str(", world!");

    // Connecting Ownership between Compile-time and Runtime

    // One type of undefined behavior is use-after-free, which occurs when a variable is used after it has been freed

    let mut v = vec![1, 2, 3];

    // immutable borrow removes the W🟦 permission from v
    let n = &v[0];
    // <- n and v refer to the same data on the heap

    v.push(4);
    // <- the data on the heap is moved to a new location (because the vector is full)
    //    and the old location is freed, which invalidates the n pointer

    // println!("{n}"); error!

    // Another type of undefined behavior is double-free, which occurs when a variable is freed twice

    let v = vec![1, 2, 3];
    let v_ref: &Vec<i32> = &v;

    // Dereferences of references to non-copyable data do not have the O🟥 permission to avoid double-frees
    // let v2 = *v_ref; error!
    // <- v and v2 refer to the same data on the heap, while v_ref refers to v on the stack

    // drop(v2);
    // <- v2 is dropped, which frees the data on the heap

    // drop(v);
    // <- the data is already freed, so this would be a double-free
}
