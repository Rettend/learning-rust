use std::collections::HashMap;

fn main() {
    // Creating a New HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing Values in a HashMap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // HashMaps and Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point

    // Updating a HashMap
    // Overwriting a value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    // Adding a key and value only if a key isn’t present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    // Updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

    // Exercise 1:

    let mut numbers = vec![6, 3, -7, 1, 0, 2, 1, 5, 10];
    let mut median = 0;
    let mut mode = 0;

    numbers.sort();

    if numbers.len() % 2 == 0 {
        let middle = numbers.len() / 2;
        median = (numbers[middle] + numbers[middle - 1]) / 2;
    } else {
        let middle = numbers.len() / 2;
        median = numbers[middle];
    }

    let mut map = HashMap::new();

    for number in &numbers {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }

    let mut max = 0;

    for (key, value) in &map {
        if *value > max {
            max = *value;
            mode = **key;
        }
    }

    println!("Median: {median}, Mode: {mode}");
}
