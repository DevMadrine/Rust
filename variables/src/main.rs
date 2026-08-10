fn main() {
  let mut x = 5; //This is a mutable variable and can be changed
  println!("The value of x is: {}", x);
  x=6;
  println!("The value of x is: {}", x);
  constants();
  expressions();
  functions_with_return_types();

  println!("The value of x is: {}", functions_with_return_types());

//   conditions1();
//   conditions2();
loops();
loops1();
sum();
}

//constants are always immutable and must have a type annotation
fn constants() {
  const MAX_POINTS: u32 = 100_000; //This is a constant and cannot be changed
  println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}

fn expressions() {
  let y = {
    let x = 3;
    x + 1 //This is an expression and returns a value
  };
  println!("The value of y is: {}", y);
}

fn functions_with_return_types() -> i32 {
  return 6;//This is an expression and returns a value
  

}

// fn conditions1(){
//     let cond = true;
//     let x = if cond {1} else {2};
// }

// fn conditions2(){
//     let cond = true;
//     let x;
//     if cond{
//         x =1;
//     } else{
// x = 2;
//     }
// }

fn loops(){
    // loop{
    //     println!("This is an infinite loop");
    // }
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  
    }

};
println!("The result is: {}", result);

}


fn loops1() {
    let mut x = 0;
    'a: loop {
        x += 1;
        'b: loop {
            if x > 10 {
                continue 'a;
            } else {
                break 'b;
            }      
        }
        break;       
    }
}


fn sum() {
    let a = [5; 10];
    let mut sum = 0;
    for x in a {
        sum += x;
    }
    println!("{sum}");
}