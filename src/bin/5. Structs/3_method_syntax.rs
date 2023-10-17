#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Self) -> Self {
        Self {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // Debug Prints

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    if rect1.width() {
        println!("rect1 has width: {}", rect1.width);
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // Methods

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Associated Functions

    let square1 = Rectangle::square(3);
    println!("square1 is {:?}", square1);

    // Method Calls are Syntactic Sugar for Functions Calls

    let mut r = Rectangle {
        width: 1,
        height: 2,
    };
    let area1 = r.area();
    let area2 = Rectangle::area(&r);
    assert_eq!(area1, area2);

    r.set_width(2);
    Rectangle::set_width(&mut r, 3);

    // Referencing and Dereferencing

    let r = &mut Box::new(Rectangle {
        width: 1,
        height: 2,
    });

    let area1 = r.area();
    let area2 = Rectangle::area(&**r);
    assert_eq!(area1, area2);

    // Methods and Ownership

    let rect = Rectangle {
        width: 0,
        height: 0,
    };
    // - rect: R🟧, O🟥

    println!("{}", rect.area()); // - rect: R🟧

    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };

    let max_rect = rect.max(other_rect); // - rect: R🟧, O🟥, - other_rect: R🟧, O🟥

    // error!
    // rect.set_width(0); // - rect: R🟧, W🟦

    let mut rect = Rectangle {
        width: 0,
        height: 0,
    };
    // - rect: R🟧, W🟦, O🟥

    rect.set_width(1); // - rect: R🟧, W🟦

    let rect_ref = &rect;
    // - *rect_ref: R🟧

    // error!
    // rect_ref.set_width(2); // - rect: R🟧, W🟦

    // Moves with self
}
