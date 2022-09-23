use rand::Rng;
use std::io::Write;

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut trng = rand::thread_rng();

    let mut tries = 0;
    let mut secret_number: i32 = trng.gen_range(1..=100);

    loop {
        print!("Your guess: ");
        
        if let Err(_) = stdout.flush() {
            break;
        }

        let mut guess = String::new();

        if let Err(e) = stdin.read_line(&mut guess) {
            println!("Couldn't read line: {e}");
            break;
        }

        let guess: i32 = match guess.trim().parse() {
            Ok(g) => g,
            Err(_) => {
                println!("Please, enter a valid number.");
                continue;
            }
        }; 
        
        tries += 1;

        if guess > secret_number {
            println!("Too big!");
        } else if guess < secret_number {
            println!("Too small!");
        } else {
            println!("You won! Tries: {tries}. Press enter to play again.");
            let mut temp = String::new();
            if let Err(e) = stdin.read_line(&mut temp) {
                println!("Couldn't read line: {e}");
                break;
            } else {
                if !temp.trim().is_empty() {
                    break;
                }
                tries = 0;
                secret_number = trng.gen_range(1..=100);
            }
        }

    }
}
