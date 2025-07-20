use std::cmp::Ordering;

use rand::Rng;

fn gen_random_number(min: i32, max: i32) -> i32 {
    let num = rand::rng().random_range(min..=max);

    num // last line of function with return that value of no ';' at the end
}

pub fn guess_the_number() {
    let num = gen_random_number(0, 100);

    println!("Type 'q' to quit");

    loop {
        println!("Guess the number");

        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess.trim().eq("q") {
                    break;
                }
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed correctly!");
                break;
            }
        }
    }
}
