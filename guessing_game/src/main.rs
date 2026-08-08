use std::cmp::Ordering; //library for comparing values
use std::io; //imput output library for user input and output
use rand::Rng; //random number generator library

fn main() {
    println!("Guess the number!"); //prints a message to the console
    let secret_number = rand::thread_rng().gen_range(1..=100); //generates a random number between 1 and 100

//   println!("The secret_number is:{}", secret_number); //prints the secret number to the console

  loop{
  println!("Please input your guess.");

  let mut guess = String::new();   //creates a mutable variable called guess and sets it to a new instance of the String type

  io::stdin()
  .read_line(&mut guess) //reads the user input and stores it in the guess variable
  .expect("Failed to read line"); //handles any errors that may occur while reading the input

  let guess:u32 = match guess.trim().parse() {
    Ok(num) => num, //if the input is a valid number, store it in the guess variable
    Err(_) => continue, //if the input is not a valid number, continue to the next iteration of the loop
  };//converts the guess variable from a string to an unsigned 32-bit integer
  println!("You guessed: {}", guess);

  match guess.cmp(&secret_number){
    Ordering::Less => println!("Too small!"), //if the guess is less than the secret number, print "Too small!"
    Ordering::Greater => println!("Too big!"), //if the guess is greater than the secret number, print "Too big!"
    Ordering::Equal => {
        println!("You win!");
        break;

} //if the guess is equal to the secret number, print "You win!"
  }
}
}
