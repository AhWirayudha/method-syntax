#[derive(Debug)]
// struct Rectangle
struct Rectangle {
    width: u32,
    height: u32,
}

// method area, function that context of Rectangle *the capability of Rectangle
impl Rectangle {
    fn area(&self) -> u32 { // self is a reference to Rectangle, &self is short of self: &Self
        self.width * self.height
    }

    // we can name it same as the struct
    fn width(&self) -> bool {
        self.width > 0
    }

    // can hold, method with more parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// associated function
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", rect1.area());

    // call width method
    println!("The rectangle has a nonzero width: {} {}", rect1.width(), rect1.width);

    // method with more parameters
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // can hold
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // associated function
    let sq = Rectangle::square(3);
    println!("sq: {:?}", sq); // sq: Rectangle { width: 3, height: 3 }
    println!("sq: {:#?}", sq.area());
}
