use std::{io, cmp::Ordering};
use rand::Rng;

fn main(){
    println!("Guess the number");
    let random_number = rand::thread_rng().gen_range(1..=100);

    loop {

        println!("Please input a guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to get input");

        let guess: u32 = match guess.trim().parse(){
            Ok(val) => val,
            Err(_) => continue,
        };


        println!("You guess: {guess}");

        match guess.cmp(&random_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("yep");
                break;
            }
        }
    }
}
