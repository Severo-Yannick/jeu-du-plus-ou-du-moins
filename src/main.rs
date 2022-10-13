// The std::io module contains a number of common things you'll need when doing input and output.
use std::io;
// Rand provides utilities to generate random numbers, to convert them to useful types and distributions, and some randomness-related algorithms.
use rand::Rng;
// cmp is functionality for ordering and comparison.
use std::cmp::Ordering;

fn main() {
    println!("Devinez le nombre !");
    println!("Veuillez entrer un nombre !");
    let range = 1..101;
    let secret_number = rand::thread_rng().gen_range(range);
    println!("Le nombre secret aleatoire est: {secret_number}");
    // Create a new empty String ""
    let mut assumption = String::new();
    // Constructs a new handle to the standard input of the current process.
    io::stdin()
        .read_line(&mut assumption)
        .expect("Echec de la lecture de l'entrée utilisateur");
    
        let assumption: u32 = assumption.trim().parse().expect("Veuillez entrer un nombre !");

        println!("Votre nombre : {assumption}");
        // match control flow based on pattern matching.
        match assumption.cmp(&secret_number) {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => println!("Vous avez gagné !"),
        }
}