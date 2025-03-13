// // use std::io;
// // use rand::Rng;
// // fn main() {
// //  println!("Guess the number!");
// //  let secret_number = rand::thread_rng().gen_range(1..=100);
// //  println!("The secret number is: {secret_number}");
// //  println!("Please input your guess.");
// //  let mut guess = String::new();
// //  io::stdin()
// //  .read_line(&mut guess)
// //  .expect("Failed to read line");
// //  println!("You guessed: {guess}");
// // }


// fn main (){
//     check_age(16);
//     check_age(20);

//     call_function(|| println!("Hello from function parameter!"));

//     let numbers = vec![10, 20, 30, 40, 50];
//     println!("Average: {}", calculate_average(&numbers));
    
//     println!("30°C to Fahrenheit: {}", celsius_to_fahrenheit(30.0, true));
//     println!("86°F to Celsius: {}", celsius_to_fahrenheit(86.0, false));

// }

// fn check_age(age: u32) {
//     if age < 18 {
//         println!("You are under age");
//     } else {
//         println!("You can come in");
//     }
// }



// fn call_function<F: Fn()>(func: F){
//     func();
// }


// // fn calculate_average(numbers: &[i32]) -> f64 {
// //     let sum: i32 = numbers.iter().sum();
// //     let count = numbers.len() as f64;
// //     sum as f64 / count
// // }
// fn calculate_average(numbers: &[i32]) -> f64{
//     let count = numbers.len() as f64;
//     let mut sum: i32 = 0;

//     for num in numbers{
//         sum += num;
//     }
//     sum as f64 / count
// }

// fn celsius_to_fahrenheit(temp: f64, to_fahrenheit: bool) -> f64 {
//     if to_fahrenheit {
//         temp * 9.0 / 5.0 + 32.0
//     } else {
//         (temp - 32.0) * 5.0 / 9.0
//     }
// }



// Guessing game
// difficulty levels
// random number
// time boundaries
// Highest score

use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Read};
use std::time::{Instant, Duration};


fn main() {
    println!("Welcome to the Guessing Game!");

    let mut high_score: Option<(u32, Duration)> = None;


    let difficulty = choose_difficulty();
    let (seceret_number, range) = generate_seceret_number(difficulty);

    println!("I'm thinking of a number between 1 and {}.", range);

    let mut guess_count = 0;
    let start_time = Instant::now();

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok (num) => num, 
            Err(_) => {
                println!("Invalid input. Please enter a valid number");
                continue;
            }
        };

        guess_count += 1;

        println!("You guessed: {}", guess);

        match guess.cmp(&seceret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                let duration = start_time.elapsed();
                println!("You win! You took {} guesses and {:?} seconds.", guess_count, duration.as_secs());

                
            }
        }
    }




    
}

fn choose_difficulty() -> u32 {
    loop {
        println!("Choose a difficulty level:");
        println!("1. Easy (1 - 50)");
        println!("2. Medium (1 - 100)");
        println!("3. Hard (1 - 200)");

        let mut difficulty = String::new();

        io::stdin()
            .read_line(&mut difficulty)
            .expect("Failed to read line");

        match difficulty.trim().parse() {
            Ok(1) => return 50,
            Ok(2) => return 100,
            Ok(3) => return 200,
            _ => println!("Invalid input, Please enter a valid difficulty level (1, 2, 3)."),
        }
    }
}

fn generate_seceret_number(range: u32) -> (u32, u32) {
    let seceret_number = rand::rng().random_range(1..=range);
    (seceret_number, range)

}