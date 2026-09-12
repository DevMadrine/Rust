fn main() {
   print_value(42);
   print_value("Hello, world!");
   generic_structs();
   exercise1();
   generic_arrays();
}
//allows T to print
fn print_value<T: std::fmt::Display>(value: T) {
    println!("{}", value);
}

//generics with structs

fn generic_structs() {
    struct Point<T>{
        x:T,
        y:T,
    }

    println!("Point with integers: ({}, {})", Point { x: 5, y: 10 }.x, Point { x: 5, y: 10 }.y);
}

//exercises
fn exercise1(){
    struct Point<T, U> {
        x: T,
        y: U,
    }


    impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    Point {
        x: self.x,
        y: other.y,
    }
}
    }
    

    let point1: Point<i32, i32> = Point { x: 5, y: 10 };
    let point2: Point<&str, char> = Point { x: "Hello", y: 'c' };
    let point3: Point<i32, char> = point1.mixup(point2);

    assert_eq!(point3.x, 5);
    assert_eq!(point3.y, 'c');

    println!("Successfully mixed up points: ({}, {})", point3.x, point3.y);

    
}

//generics with Arrays
fn generic_arrays() {
    #[derive(Debug)]
    struct Array<T, const N: usize> {
        data: [T; N],
    }

    let arrays: [Array<i32, 3>; 3] = [
        Array { data: [1, 2, 3] },
        Array { data: [4, 5, 6] },
        Array { data: [7, 8, 9] },
    ];

    let floats: [Array<f64, 2>; 1] = [
        Array { data: [1.1, 2.2] },
       
    ];

    println!("Array of arrays: {:?}", arrays);
    println!("Float arrays: {:?}", floats);
}


