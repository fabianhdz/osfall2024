use std::io;

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        return 0;
    } else if guess > secret {
        return 1;
    } else {
        return -1;
    }
}

fn main() {
    let mut secret_number: i32 = 10;
    let mut guess_counter: i32 = 0;

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read guess number");

        let guess: i32 = input
            .trim()
            .parse()
            .expect("Please enter a valid integer number");
        guess_counter += 1;
        let result = check_guess(guess, secret_number);
        if result == 0 {
            println!("Guess was correct!!!");
            break;
        } else if result == 1 {
            println!("Guess was too high!");
        } else {
            println!("Guess was too low!");
        }
    }
    println!("It took {} guesses", guess_counter);
}
