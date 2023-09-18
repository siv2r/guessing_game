use std::io::{self, Write};
use rand::Rng;

fn generate_secret() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=100)
}

fn get_input() -> i32 {
    let mut inp = String::new();

    print!("Please enter your guess: ");
    io::stdout().flush().unwrap();

    let _result  = match io::stdin().read_line(&mut inp) {
        Ok(n) => n,
        Err(_) => panic!("Couldn't read user input"),
    };

    match inp.trim().parse::<i32>() {
        Ok(guess) => guess,
        Err(_) => -1, // to make this fn return i32
    }
}

fn main() {
    let secret: i32 = generate_secret();

    loop {

        let inp: i32 = get_input();

        // handle errors
        if inp < 0 || inp > 100 {
            println!("Please enter a valid guess (0..100)");
            continue;
        }

        // check against secret value
        if inp > secret {
            println!("Guess is too high");
        } else if inp < secret {
            println!("Guess is too low");
        } else {
            println!("Congratulations, you guessed the secret!!");
            break;
        }
    }
}
