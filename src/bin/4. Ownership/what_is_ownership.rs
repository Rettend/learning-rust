fn read(y: bool) {
    if y {
        println!("y is true");
    }
}

fn main() {
    // you cannot switch these two lines
    let x = true;
    read(x);

    // Stack and Heap
    // Stack: LIFO, fixed size, fast
    // Heap: dynamic size, slower

    let a = 5; // Stack: a = 5
    let mut b = a; // Stack: a = 5, b = 5
    b = 6; // Stack: a = 5, b = 6

    // This copies the value of a to b

    let a = [0; 1_000_000]; // Stack: a = [0, 0, 0, ...] (1 million zeros)
    let b = a; // Stack: a = [0, 0, 0, ...], b = [0, 0, 0, ...] (2 million zeros)

    // Boxex live in the Heap
    // Transfer access to data without copying it

    let a = Box::new([0; 1_000_000]); // Stack: a = Box([0, 0, 0, ...]) (1 million zeros)
    let b = a; // Stack: b = Box([0, 0, 0, ...]) (1 million zeros) (b now owns the heap memory)

    // Rust does not allow manual memory management
    // Instead, it automatically frees a box's heap memory

    let a_num = 4; // 1. Stack: a_num = 4
    fn make_and_drop() {
        let a_box = Box::new(5); // 2. Stack: a_num = 4, a_box -> Heap: 5
    }
    make_and_drop(); // 3. Stack: a_num = 4
                     // a_box pointed to a heap memory that was freed by Rust

    // Collections Use Boxes
    // like Vec<T>, String, and HashMap<K, V>

    let first = String::from("Ferris"); // 1. Stack: first -> Heap: "Ferris"
    fn add_suffix(mut name: String) -> String {
        name // 2. Now, name has the string "Ferris", the data is not copied only the pointer to it
            .push_str(" Jr."); // 3. Stack name -> Heap: "Ferris Jr.", it allocates new, larger portion of memory and frees the old one
        name
    }
    let full = add_suffix(first); // 4. Stack: Full -> Heap: "Ferris Jr."
    
    // Variables cannot be used after they are moved (something has taken ownership of their value)
    // println!("{full}, originally {first}"); // error: borrow of moved value: `first`

    // Cloning Avoids Moves
    let first = String::from("Ferris");
    let first_clone = first.clone(); // 1. Stack: first -> Heap: "Ferris", first_clone -> Heap: "Ferris"
    let full = add_suffix(first_clone); // 2. Stack: First -> Heap: "Ferris", full -> Heap: "Ferris Jr."
    println!("{full}, originally {first}");

    
}
