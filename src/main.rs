use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("Guessing Game!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut counter = 0;
    loop {
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        counter += 1;
        let guess: u32 = guess.trim().parse().expect("Enter a number");

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win, it took you {} guesses!", counter);
                break;
            }
        }
    }
}
