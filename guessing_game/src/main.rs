use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("---------Guess the number!---------");
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Guess a number:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) //Appends to whatever string it is passed
            .expect("Failed to read line."); //On error, panics if called on an Err value

        let guess: u32 = match guess.trim().parse(){ //Match statement is good for Result returns
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break; //Not the match statement, the loop
            }
        }
    }   
}
