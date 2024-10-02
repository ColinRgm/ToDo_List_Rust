use std::fs::OpenOptions;
use std::io::{self, Write};

// For the flags
use std::path::PathBuf;
use clap::{arg, command, value_parser, ArgAction, Command};


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
        eprintln!("ToDo ajoutée au fichier");              // if it works
    }

    // -------------------------------------------------------------------- Création des drapeaux --
    let matches = command!()
        .arg(arg!([name] "Option"))
        .arg(
            arg!(
                -c --config <FILE>
            )
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(
            -d --debug ... " Debugging"
        ))
        .subcommand(
            Command::new("Test")
                .about("Testing things")
                .arg(arg!(-l --list "Lists test value").action(ArgAction::SetTrue)),
        )
        .get_matches();
}












