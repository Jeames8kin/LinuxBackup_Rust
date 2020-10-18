use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("le game of guess le number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

//    println!("The secret number is: {}", secret_number);

    loop {
        
        println!("Enter your guess.");

        let mut guess = String::new();                                      // Mutable variable; can be changed. Not assigning mut will make it immutable; can't be changed.

        io::stdin()
            .read_line(&mut guess)
            .expect("Error 2: Unknown character/phrase entered.");

        println!("You guessed: {}", guess);                                 // Curly brackets will include any variable you put out the string in order. 

        let guess: u32 = match guess.trim().parse() {                       // Removes the new line (\n) since stdin will take the Enter into account. Moving from an expect call (where it crashes the program, no way to avoid) to a match expression to check is a way to handle an error.
            Ok(num) => num,
            Err(_) => continue,
        }; 
            

        match guess.cmp(&secret_number) {                                   // Uses std::cmp::Ordering to check to see if the above variable and the one in the parameter are the same.
            Ordering::Less => println!("Too small, try again."),
            Ordering::Greater => println!("Too large, try again."),         // Testing Git.
            Ordering::Equal => {
                println!("Bang on mate.");
                break;

            }

        }

    }

}   