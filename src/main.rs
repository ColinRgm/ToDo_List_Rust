use std::fs::OpenOptions;
use std::io::{self, Write};


fn main() {
    println!("Texte à ajouter dans le fichier txt");

    add_text_in_text_file();
}

// Function that add the text you write in command line in the txt files
fn add_text_in_text_file() {

    /* ---------------------------- Lecture de l'entrée utilisateur ---------------------------- */
    // Creation of a mutable variable that will store the user entry
    let mut text = String::new();

    // This function will read the entry
    io::stdin()
        .read_line(&mut text)
        .expect("Erreur");





    /* --------------------------- Ajout du texte dans le fichier txt --------------------------- */
    // Get file
    let path = ("todo.txt");


    let mut file = OpenOptions::new()
        .append(true) // Add text
        .create(true) // Create the file if not exist
        .open(path) // Open the file
        .expect("Pas de fichier"); // If error

    // Write the text if can't error e will occure
    if let Err(e) = writeln!(file, "\n{}", text) {
        // if error
        eprintln!("Erreur : {}", e);
    } else {
        // if it works
        eprintln!("Texte ajouté");
    }
}