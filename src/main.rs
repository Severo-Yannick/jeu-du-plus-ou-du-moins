// The std::io module contains a number of common things you'll need when doing input and output.
use std::io;
// Rand provides utilities to generate random numbers, to convert them to useful types and distributions, and some randomness-related algorithms.
use rand::Rng;
// cmp is functionality for ordering and comparison.
use std::cmp::Ordering;

fn main() {
    println!("Devinez le nombre !");
    let range = 1..101;
    let secret_number = rand::thread_rng().gen_range(range);
    let mut _score: u8 = 0;
    let _win: &str = "\u{2705} Vous avez gagné !";
    let _minus: &str = "\u{2796} C'est moins !";
    let _plus: &str = "\u{2795} C'est plus !";
    let _result: &str = "\u{1F449} Votre scrore :";
    // println!("Le nombre secret aleatoire est: {secret_number}");
    
    loop {
        _score += 1;
        println!("Veuillez entrer un nombre !");
        // Create a new empty String ""
        let mut assumption = String::new();
        // Constructs a new handle to the standard input of the current process.
        io::stdin()
            .read_line(&mut assumption)
            .expect("Echec de la lecture de l'entrée utilisateur");
        
            let assumption: u32 = match assumption.trim().parse() {
                Ok(number) => number,
                Err(_) => continue,
            };
    
            println!("Votre nombre : {assumption}");
            // match control flow based on pattern matching.
            match assumption.cmp(&secret_number) {
                Ordering::Less => println!("{}", _plus),
                Ordering::Greater => println!("{}", _minus),
                Ordering::Equal => {
                    println!("{}", _win);
                    println!("{_result} {_score}");
                    break;
                }
            }
    }
}