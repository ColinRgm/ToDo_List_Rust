use std::string::String;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use clap::{command, Arg, ArgAction};
use std::env;

fn main() {
    flags();
}


// ------------------------------------------------------------------ Drapeaux et leurs fonctions --
fn flags() {


    // -------------------------------------------------------------------- Création des drapeaux --
    let _arguments = command!()
        .about("Welcome to your personnal ToDo list ! \n
                PLease do not delete line 0")
        .arg
        (
            // Delete flag
            Arg::new("delete")
                .short('d') // Short Name
                .long("delete") // Long Name
                .help("To delete a todo") // Description
                .action(ArgAction::Append)
                .conflicts_with("add") // To avoid conflict between flags
        )
        .arg
        (
            // Add flag
            Arg::new("add")
                .short('a')
                .long("add")
                .help("To add a todo")
                .action(ArgAction::SetFalse)
                .conflicts_with("delete")
        )
        // Build the instance
        .get_matches();

    let args: Vec<String> = env::args().collect();

    let input = args[2].clone();

    // println!("{}", input);


    // --------------------------------------------------------- Appel des fonctions des drapeaux --
    if input == "1" {
        println!("Suppression de la ligne {}", input)
    } else if input == "2" {}

    // ------------------------------------------------------------- Afficher la liste des tâches --
    // println!("Liste des tâches à faire : {}", arguments.get_one::<String>("list").unwrap());
}
/*
fn test() {
    println!("Delete fonctionne !")
}

fn test2() {
    println!("Add fonctionne !")
}

 */


// --------------------------------------------------------------------------------- Ajouter todo --
fn _add_todo() {
    println!("Texte à ajouter dans le fichier txt");


    // ---------------------------------------------------------- Lecture de l'entrée utilisateur --
    let mut text = String::new(); // Variable mutable qui stocke l'entrée de l'utilisateur

    io::stdin()
        .read_line(&mut text)
        .expect("Erreur"); // Lire l'entrée


    // ------------------------------------------------------- Ajout du texte dans le fichier txt --
    let path = "todo.txt"; // Récupérer le fichier

    let mut file = OpenOptions::new()
        .append(true) // Ajout du texte
        .create(true) // Créer un fichier si il n'existe pas
        .open(path) // Ouvrir le fichier h
        .expect("Pas de fichier"); // if error


    // --------------------------------------------------------------------- Retourner une erreur --
    if let Err(e) = writeln!(file, "{}", text) {
        eprintln!("Erreur : {}", e); // if error
    } else {
        eprintln!("ToDo ajoutée au fichier: {}", path); // if it works
    }
}


// -------------------------------------------------------------------------- Supprimer les todos --
fn _delete_todo() {
    /*
        1. Récupèrer le fichier - DONE
        2. Récupèrer le  numéro de ligne - DONE
        3. Effacer la ligne entré en argument du drapeau
        4. Fermer le programme
    */


    // ------------------------------------ Récupérer le fichier et retourner le numéro de lignes --
    fn get_line() -> io::Result<()> {
        let path = Path::new("todo.txt"); // get file

        let file = File::open(&path)?;
        let read = BufReader::new(file);

        for (index, line) in read.lines().enumerate() {
            let line_num = line?;

            println!("Ligne {}: {}", index, line_num);
        }

        Ok(())
    }


    // Récupérer le numéro de la ligne entrée par l'utilisateur
    // Supprimer la ligne souhaitée

    let _ = get_line();


    // println!("Deleting a todo...")
}