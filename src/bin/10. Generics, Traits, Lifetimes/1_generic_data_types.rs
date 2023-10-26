fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    // In Function Definitions
    fn largest<T>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list.iter() {
            // error[E0369]: binary operation `>` cannot be applied to type `T`
            // if item > largest {
            //     largest = item;
            // }
        }

        largest
    }
}