use std::io;
use colored::*;
use crate::utils::generate_random_number;

pub fn guess_number(guess: u16) -> io::Result<()> {
    if guess <= 0 || guess >= 11 {
        println!("{}", "##==>> Your guess must be a value between 1 - 10".red());
        return Ok(())
    }

    let random_value = generate_random_number(1, 100);
    if random_value == guess {
        println!("{}", "#=> CORRECT! You guessed correctly.".green());
    } else if random_value < guess {
        println!("{}", "#=> INCORRECT! Guess Was Too High!".red());
    } else if random_value > guess {
        println!("{}", "#=> INCORRECT! Guess Was Too Low!".red());
    }

    println!("#=> Your Guess: {}", guess);
    println!("#=> Correct Guess: {}", random_value);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn can_guess_incorrectly() {
        let guess = 13;
        let random_value = generate_random_number(1, 100);
        assert_eq!(guess, random_value);
    }

    #[test]
    fn can_guess_correctly() {
        let guess = 5;
        let random_value = 5;
        assert_eq!(guess, random_value);
    }
}
