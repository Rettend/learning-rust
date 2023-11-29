fn main() {
    // Creating a Vector
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    // Updating a Vector
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Reading Elements of Vectors
    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("The third element is {third}");

    let third = v.get(2);
    if let Some(third) = third {
        println!("The third element is {third}");
    }

    // let does_not_exist = &v[100]; // PANIC!
    let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6); // error: cannot borrow `v` as mutable because it is also borrowed as immutable

    println!("The first element is: {first}");

    // Iterating over the Values in a Vector
    let v = vec![100, 32, 57];
    for n_ref in &v {
        let n_plus_one = n_ref + 1; // implicit deref coercion
        println!("{n_plus_one}");
    }

    let mut v = vec![100, 32, 57];
    for n_ref in &mut v {
        *n_ref += 50;
    }

    // Safely Using Iterators
    fn dup_in_place(v: &mut Vec<i32>) {
        for n_ref in v.iter() { // v.iter() removes W🟦 permission
             // - *v: R🟧

            // error!
            // v.push(*n_ref); // - *n_ref: R🟧, W🟦
        }
    }

    // Using an Enum to Store Multiple Types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Dropping a Vector Drops Its Elements
    {
        let vector = vec![1, 2, 3, 4];
    } // <- vector goes out of scope and is freed here
      // println!("{vector:?}"); error!

    let n: &i32;
    {
        let vector = vec![1, 2, 3, 4];

        n = &vector[3];
    } // <- vector goes out of scope and is freed here
      // println!("{n}"); // error: borrowed value does not live long enough
}
