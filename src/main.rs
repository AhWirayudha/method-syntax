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
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", rect1.area());

    // call width method
    println!("The rectangle has a nonzero width: {}", rect1.width());
}
