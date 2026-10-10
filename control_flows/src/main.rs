fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    test();
    multiple_conditions();
    if_let();
    returning_loops();
    loops();

}


//conditional statements in Rust are similar to other programming languages. The `if` statement checks a condition, and if it evaluates to true, the code block inside the `if` statement is executed. If the condition is false, the code block inside the `else` statement is executed instead. In this example, since `number` is 3, which is less than 5, the output will be "condition was true".

// condition must be a boolean expression, meaning it must evaluate to either true or false. In Rust, you cannot use non-boolean values (like integers) directly as conditions, which is different from some other languages where non-zero values are treated as true.

fn test() {
    let number = 3;

    if number < 5 {
        println!("number was three");
    }
}


//handling multiple conditions can be done using `else if` statements. This allows you to check multiple conditions in sequence. For example:

fn multiple_conditions() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

//Using `if` in a `let` statement allows you to assign a value based on a condition. This is useful for initializing variables with different values depending on certain conditions. For example:

fn if_let() {
    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}



// returning values from loops can be done using the `break` statement with a value. This allows you to exit the loop and return a value that can be assigned to a variable. For example:

fn returning_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}


//repitition in Rust can be achieved using loops. The `loop` keyword creates an infinite loop, which can be exited using the `break` statement. For example:
// fn loops(){
//     loop {
//         println!("again!");
    
//     }
// }

//


fn loops() {
    let a = [5; 10];
    let mut sum = 0;
    for x in a {
        sum += x;
    }
    println!("{sum}");
}