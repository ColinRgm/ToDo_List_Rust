use std::string::String;
use std::fs::OpenOptions;
use std::io::{self, Write};

// For the flags
use clap::Parser;



// ------------------------------------------------------------------------ Création des drapeaux --
struct Args {
    delete: bool, // delete a todo
}


fn main() {
    println!("Texte à ajouter dans le fichier txt");

    add_text_in_text_file();
}


/* Function that add the text you write in command line in the txt files */
fn add_text_in_text_file() {

    // ---------------------------------------------------------- Lecture de l'entrée utilisateur --
    // Creation of a mutable variable that will store the user entry
    let mut text = String::new();


    // This function will read the entry
    io::stdin()
        .read_line(&mut text)
        .expect("Erreur");



    // ------------------------------------------------------- Ajout du texte dans le fichier txt --
    let path = "todo.txt";                  // get file


    let mut file = OpenOptions::new()
        .append(true)           // add text
        .create(true)           // create the file if not exist
        .open(path)                 // open the file
        .expect("Pas de fichier");         // if error

    
    // Write the text, if can't error e will occure
    if let Err(e) = writeln!(file, "\n{}", text) {
        eprintln!("Erreur : {}", e);            // if error
    } else {
        eprintln!("ToDo ajoutée au fichier");   // if it works
    }



    // ----------------------------------------------------------------- Utilisation des drapeaux --
    let args = Args::parse();

    if args.delete {
        eprintln!("Vous souhaitez supprimer une tâches");
    }
}











