mod associated_functions;

fn main() {
    methods();
    associated_functions::associated_functions();
}

fn methods(){
    struct Rectangle{
        width:u32,
        height:u32,
    }

    let rect1 = Rectangle{width:30, height:50};

    impl Rectangle{
        fn area(&self) -> u32{
            self.width * self.height
        }
    }

    println!("The area of the rectangle is {} square pixels.", rect1.area());
}



