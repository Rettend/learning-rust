fn main() {}

// Unit Tests

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

// Integration Tests
// in the tests/ directory

// use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}

