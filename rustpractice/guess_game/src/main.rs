use rand::Rng;

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

    let secret = rand::thread_rng().gen_range(1..10);
    let mut attempts = 0;

    loop {

        attempts += 1;
        let guess = rand::thread_rng().gen_range(1..10);
        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Congrats, you guessed the secret number!");
            break;
        } else if result == 1 {
            println!("Sorry, your guess is too high. Try again!");
        } else {
            println!("Sorry, your guess is too low. Try again!");
        }

    }

    println!("It took you {} attempts to guess the secret number!", attempts);

}