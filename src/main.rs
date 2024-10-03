use std::error::Error;
use std::fs;
use std::string::String;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
// For the flags
use clap::{command, Arg};


fn main() {
    flags();

    add_todo();

    delete_todo();
}




// ------------------------------------------------------------------ Drapeaux et leurs fonctions --
fn flags() {


    // -------------------------------------------------------------------- Création des drapeaux --
    let arguments = command!()
        .about("Welcome to your personnal ToDo list !")
        .arg(
            Arg::new("delete")
                .short('d')
                .long("delete")
                .help("To delete a todo").takes_value(true)
                // .conflicts_with() Pour éviter d'appeler plusieurs drapeaux en même temps
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();


    // ------------------------------------------------------ Appel de la fonction de suppression --
    if arguments.get_one::<bool>("delete") == Some(&true) {
        delete_todo();
    }


    // ------------------------------------------------------------- Afficher la liste des tâches --
    // println!("Liste des tâches à faire : {}", arguments.get_one::<String>("list").unwrap());
}




// -------------------------------------------------------------------------- Supprimer les todos --
fn delete_todo() {

    /*
    1. Récupèrer le fichier - DONE
    2. Récupèrer le  numéro de ligne - DONE
    3. Effacer la ligne complète
    4. Fermer le programme
    */


    // ------------------------------------ Récupérer le fichier et retourner le numéro de lignes --
    fn get_line() -> io::Result<()> {
        let path = Path::new("todo.txt"); // get file

        let file = File::open(&path)?;
        let read = BufReader::new(file);

        for (index, line) in read.lines().enumerate() {
            let line_num = line?;

            println!("Ligne {}: {}", index + 1, line_num);
        }

        Ok(())
    }

    get_line();

    // println!("Deleting a todo...")
}




// --------------------------------------------------------------------------------- Ajouter todo --
fn add_todo() {
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


    // --------------------------------------------------------------------- Retourner une erreur --
    if let Err(e) = writeln!(file, "{}", text) {
        eprintln!("Erreur : {}", e); // if error
    } else {
        eprintln!("ToDo ajoutée au fichier: {}", path); // if it works
    }
}











