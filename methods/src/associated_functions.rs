//associated functions are functions that are defined within the context of a struct, enum, or trait, but do not take a self parameter. They are called using the double colon syntax (::) and are often used for constructors or utility functions related to the type.

pub fn associated_functions() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn new(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }
    }

    let rect1 = Rectangle::new(30, 50);
    println!("The rectangle has width {} and height {}.", rect1.width, rect1.height);
}