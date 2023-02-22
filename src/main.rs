mod game;

use crate::simple_user_input::get_input;
use crate::game::Game;

fn main() {
    let mut game = Game::default();
    println!("Bienvenue dans le juste prix. Essayez de deviner le nombre !");

    loop {
        let input = get_input("Veuillez saisir un nombre :");
        game.increment_tries();

        println!("Vous avez saisi : {input}; essai {}", game.number_of_tries);

        match input {
            _ if input == game.number => break,
            _ if input < game.number => println!("C'est plus !"),
            _ => println!("C'est moins !"),
        }
    }

    println!("Bravo, vous avez deviné le nombre {} en {} essais", game.number, game.number_of_tries);
}

mod simple_user_input {
    use std::io;

    pub fn get_input(prompt: &str) -> u32 {
        println!("{}", prompt);
        loop {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_goes_into_input_above) => {}
                Err(_no_updates_is_fine) => {}
            }
            let entry = input.trim().parse();

            if let Ok(entry) = entry {
                return entry;
            } else {
                println!("Erreur : {prompt}")
            }
        }
    }
}