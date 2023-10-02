fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");

    fn greet(g1: String, g2: String) {
        println!("{}, {}!", g1, g2);
    }
    greet(m1, m2);
    // let s = format!("{} {}", m1, m2); // error: m1 and m2 are moved

    // Alternative greet that returns ownership
    let m1 = String::from("Hello");
    let m2 = String::from("world");

    fn greet2(g1: String, g2: String) -> (String, String) {
        println!("{}, {}!", g1, g2);
        (g1, g2)
    }
    let (m1, m2) = greet2(m1, m2);
    println!("{} {}", m1, m2);

    // Or in a more concise way
    // Alternative greet that uses references
    fn greet3(g1: &String, g2: &String) {
        // g1 and g2 are references to Strings
        println!("{}, {}!", g1, g2);
    }
    greet3(&m1, &m2); // greet3 "borrows" m1 and m2
    println!("{} {}", m1, m2);
    // References are non-owning pointers

    // Dereferencing a Pointer Accesses Its Data
    // The dereference operator (*) allows us to follow a reference to the data it refers to.
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x; // "unbox" x, copy the value to a (from Box<i32> to i32)
    *x += 1; // dereference x, increment the value it refers to so x is now 2

    let r1: &Box<i32> = &x; // r1 is a reference to x on the stack
    let b: i32 = **r1; // dereference r1 twice to get the value it refers to

    let r2: &i32 = &*x; // r2 points to the heap value 2 directly
    let c: i32 = *r2; // so we only need to dereference once to get the value

    println!("{a} {b} {c}");

    // Rust can implicitly dereferences and references for us
    // Calling a mehtod with the dot (.) syntax implicitly dereferences or references a pointer as needed.
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs(); // implicit dereference
    assert_eq!(x_abs1, x_abs2, "{x_abs1} should be equal to {x_abs2}");

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs(); // implicit dereference
    assert_eq!(r_abs1, r_abs2, "{r_abs1} should be equal to {r_abs2}");

    let s = String::from("Hello");
    let s_len1 = String::len(&s); // explicit reference
    let s_len2 = s.len(); // implicit reference
    assert_eq!(s_len1, s_len2, "{s_len1} should be equal to {s_len2}");

    // println! can also implicitly convert

    // Rust Avoids Simultaneous Aliasing and Mutation
    // Accessing the same data through different variables, it is harmless
    // The problem is when we have both aliasing and mutation
    let mut v: Vec<i32> = vec![1, 2, 3]; // Stack v -> Heap [1, 2, 3], cap: 3, len: 3
    v.push(4); // Stack v -> Heap [1, 2, 3, 4], cap: 6, len: 4

    // Vectors have a capacity, so here Rust needs to allocate a new array on the heap, copy the old values to the new array, and then free the old array.

    let mut v: Vec<i32> = vec![1, 2, 3];
    let _num: &i32 = &v[2]; // Stack v -> Heap [1, 2, 3], num -> Heap 3
    v.push(4); // Stack v -> Heap [1, 2, 3, 4], num is invalidated and dropped!

    // println!("The third element is {}", *_num); // error: immutable borrow later used here

    // References Change Permissions on Paths
    // Variables have are 3 kinds of permissions on their data:
    // - 🟧 Read: data can be copied to another location
    // - 🟦 Write: data can be mutated in-place
    // - 🟥 Own: data can be moved or dropped

    // The default permission is 🟧 Read, 🟥 Own
    // When we add the 'mut' keyword, we add 🟦 Write

    // The key idea is the references can temporarily remove these permissions
    let mut v: Vec<i32> = vec![1, 2, 3];
    // - v: 🟧 Read, 🟦 Write, 🟥 Own

    let num: &i32 = &v[2];
    // - v: 🟧 Read
    // - num: 🟧x
}
