use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    array_and_tuple();

    //
    // println!("Guess the number");
    // let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);
    //
    // loop {
    //     let mut guess = String::new();
    //
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");
    //
    //
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => {
    //             println!("input must be a number");
    //             continue;
    //         }
    //     };
    //
    //     println!("You guessed: {}", guess);
    //
    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }
}

fn array_and_tuple() {
    let (x, y): (&str, i32) = ("eccomi", 25);
    println!("the tuple is {}", x);

    let array: [i32; 6] = [1, 2, 3, 4, 5, 7];
    println!("the array is {}", array[3]);
}
