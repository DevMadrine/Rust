use std::ops;

fn main() {
 trait Animal{
    fn sound(&self) -> String;
 }

 struct Cow;
 struct Sheep;

 impl Animal for Cow {
    fn sound(&self) -> String {
        String::from("Moo")
    }
 }

 impl Animal for Sheep {
    fn sound(&self) -> String {
        String::from("Baa")
    }
 }

 println!("Cow sound: {}", Cow.sound());
 println!("Sheep sound: {}", Sheep.sound());

let dog = return_dog();
    let cat = return_cat();

    println!("Dog sound: {}", dog.role());
    println!("Cat sound: {}", cat.role());

 assert_eq!(6, multiply(2u8, 3u8));
 assert_eq!(5.0, multiply(1.0, 5.0));
 assert_eq!(Foo + Bar, FooBar); 
 assert_eq!(Bar - Foo, BarFoo);
 println!("Success!");

}

//Return types that implement a trait can be used as return types in functions. This allows for polymorphism, where different types can be returned from the same function as long as they implement the same trait.Is use

   trait Animals{
       fn role(&self) -> String;
   }
   struct Dog;
   struct Cat;

   impl Animals for Dog{
       fn role(&self) -> String {
        String::from("Protection")
    }
   }
   impl Animals for Cat{
      fn role(&self) -> String {
        String::from("Companionship")
    }
   }

   fn return_dog() -> impl Animals {
      Dog
   }

   fn return_cat() -> impl Animals {
      Cat
   }



fn multiply<T: std::ops::Mul<Output = T>> (a: T, b: T) -> T {
   a * b
}

struct Foo;
struct Bar;

#[derive(Debug, PartialEq)]
struct FooBar;
#[derive(Debug, PartialEq)]
struct BarFoo;

impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> Self::Output {
        FooBar
    }
}

impl ops::Sub<Foo> for Bar {
    type Output = BarFoo;
    fn sub(self, _rhs: Foo) -> Self::Output {
        BarFoo
    }
}

impl ops:: Sub<Bar> for Foo {
    type Output = FooBar;
    fn sub(self, _rhs: Bar) -> Self::Output {
        FooBar
    }
}