use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    print!("Guessing Game!\n");

    let goal_number = rand::rng().random_range(1..=100);

    loop {
        print!("Enter Your Guess:\n");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Please input a number\n");
                continue;
            }
        };

        match guess.cmp(&goal_number) {
            Ordering::Less => print!("Too small\n"),
            Ordering::Equal => {
                print!("Correct\n");
                break;
            }
            Ordering::Greater => print!("Too big\n"),
        }
    }
}
