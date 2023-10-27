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
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            // restricting T to the PartialOrd trait is needed because of the > operator
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // In Struct Definitions
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // error!
    // let wont_work = Point { x: 5, y: 4.0 };

    struct Point2<T, U> {
        x: T,
        y: U,
    }

    let integer_and_float = Point2 { x: 5, y: 4.0 };

    // In Enum Definitions
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // In Method Definitions
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p1 = Point { x: 5.0, y: 10.0 };
    let p2 = Point { x: 5, y: 10 };

    println!("p1.distance_from_origin = {}", p1.distance_from_origin());

    // error!
    // println!("p2.distance_from_origin = {}", p2.distance_from_origin());

    // In Method Definitions with Multiple Generic Data Types
    struct Point3<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl <X1, Y1> Point3<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
            Point3 {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Performance of Code Using Generics
    let ineger = Some(5);
    let float = Some(5.0);

    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_f64 {
        Some(f64),
        None,
    }

    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
