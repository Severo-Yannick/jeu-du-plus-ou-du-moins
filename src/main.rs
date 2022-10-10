// The std::io module contains a number of common things you'll need when doing input and output.
use std::io;

fn main() {
    println!("Devinez le nombre !");
    println!("Veuillez entrer un nombre !");
    // Create a new empty String ""
    let mut supposition = String::new();
    // Constructs a new handle to the standard input of the current process.
    io::stdin()
        .read_line(&mut supposition)
        .expect("Echec de la lecture de l'entrée utilisateur");

        println!("Votre nombre : {supposition}");
}