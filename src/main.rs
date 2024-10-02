use std::string::String;
use std::fs::OpenOptions;
use std::io::{self, Write};

// For the flags
use clap::{command, Arg};


fn main() {
    initalize_flags();

    add_text_in_text_file();
}


fn initalize_flags() {

    // -------------------------------------------------------------------- Création des drapeaux --
    let arguments = command!()
        .about("Welcome to your personnal ToDo list !")
        .arg(
            Arg::new("delete")
                .short('d')
                .long("delete")
                .help("To delete a todo")
                // .conflicts_with() Pour éviter d'appeler plusieurs drapeaux en même temps
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();


    // ------------------------------------------------------ Appel de la fonction de suppression --
    if arguments.get_one::<bool>("delete") == Some(&true) {
        delete_todo();
    }

    // println!("Liste des tâches à faire : {}", arguments.get_one::<String>("list").unwrap());
    // Pour afficher la liste des tâches
}

fn delete_todo() {

    /*
    1. Récupèrer le fichier
    2. Récupèrer le  numéro de ligne
    3. Effacer la ligne complète
    4. Fermer le programme
    */

    let path = "todo.txt"; // get file


    println!("Deleting a todo...")
}


fn add_text_in_text_file() {

    println!("Texte à ajouter dans le fichier txt");

    // ---------------------------------------------------------- Lecture de l'entrée utilisateur --
    let mut text = String::new(); // Creation of a mutable variable that will store the user entry

    io::stdin()
        .read_line(&mut text)
        .expect("Erreur"); // This function will read the entry


    // ------------------------------------------------------- Ajout du texte dans le fichier txt --
    let path = "todo.txt"; // get file


    let mut file = OpenOptions::new()
        .append(true) // add text
        .create(true) // create the file if not exist
        .open(path) // open the file
        .expect("Pas de fichier"); // if error


    // This condition will read the entry and add it to the file or occure an error
    if let Err(e) = writeln!(file, "\n{}", text) {
        eprintln!("Erreur : {}", e); // if error
    } else {
        eprintln!("ToDo ajoutée au fichier"); // if it works
    }
}











