use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    guess_the_number();
}

fn guess_the_number(){
    println!("Guess the right number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess:i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Secret number is higher"),
            Ordering::Greater => println!("Secret number is lower"),
            Ordering::Equal => {
                println!("You've guessed the right number!");
                break;
            }
        }
    }
}