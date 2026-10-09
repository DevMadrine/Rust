fn main() {
    let result = plus_one(5, 6);
    println!("The value of result is: {}", result);
    another_function();
    let x: i32 = five();
    println!("The value of x is: {}", x);


    let a: i32 = {
    let b: i32 = 3;
    b + 1
};
println!("The value of a is: {}", a);

statements();

}

fn another_function(){
    println!("Another function called!");
}

fn five() -> i32 {
    5
}


//parameters and arguments

fn plus_one(u: i32, v: i32) -> i32 {
  u + v
}

// statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value. The main difference between statements and expressions is that statements do not return a value, while expressions do.


fn statements(){
    let x: i32 ={
        let y = 6;//statement
        y + 1//expression
};

println!("The value of x is: {}", x);
}

fn f(x:i32) -> i32 {
    x + 1
}
