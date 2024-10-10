use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use clap::{Command, Arg, ArgAction};
use std::{env, fs};


// ----------------------------------------------------------- Fonction servant à faire des tests --
fn _function_de_test() {
    println!("Ceci est un test")
}

fn main() {

    // -------------------------------------------------------------------- Creation des drapeaux --
    let argument = Command::new("myflags")
        .about("Welcome to your personnal ToDo List")
        .arg
        (
            // Add flag
            Arg::new("add")
                .short('a') // Short Name
                .long("add") // Long Name
                .help("Add a todo") // Description
                .action(ArgAction::SetTrue)
        )
        .arg
        (
            // Delete flag
            Arg::new("delete")
                .short('d') // Short Name
                .long("delete") // Long Name
                .help("Delete a todo") // Description
                .action(ArgAction::SetTrue)
        )
        .arg
        (
            // Done flag
            Arg::new("done")
                .short('o') // Short Name
                .long("done") // Long Name
                .help("Mark a todo done") // Description
                .action(ArgAction::SetTrue)
        )
        .arg
        (
            // Undone flag
            Arg::new("undone")
                .short('u') // Short Name
                .long("undone") // Long Name
                .help("Mark a todo undone") // Description
                .action(ArgAction::SetTrue)
        )
        .arg
        (
            // Due flag
            Arg::new("due")
                .short('e') // Short Name
                .long("due") // Long Name
                .help("Todo to do") // Description
                .action(ArgAction::SetTrue)
        )
        .arg
        (
            // List flag
            Arg::new("list")
                .short('l') // Short Name
                .long("list") // Long Name
                .help("List the todo") // Description
                .action(ArgAction::SetTrue)
        )
        .get_matches(); // Build the instance

}


// --------------------------------------------------------------------------------- Todo ajoutée --
fn _add() {
    println!("Texte à ajouter dans le fichier txt");


    // Lecture de l'entrée utilisateur
    let mut text = String::new(); // Variable mutable qui stocke l'entrée de l'utilisateur

    io::stdin()
        .read_line(&mut text)
        .expect("Erreur"); // Lire l'entrée


    // Ajout du texte dans le fichier txt
    let path = "todo.txt"; // Récupérer le fichier

    let mut file = OpenOptions::new()
        .append(true) // Ajout du texte
        .create(true) // Créer un fichier si il n'existe pas
        .open(path) // Ouvrir le fichier
        .expect("Pas de fichier"); // Message en cas d'erreur


    // Retourner une erreur
    if let Err(e) = writeln!(file, "{}", text) {
        eprintln!("Erreur : {}", e); // Si erreur
    } else {
        eprintln!("ToDo ajoutée au fichier: {}", path); // Si tout fonctionne
    }
}


// ------------------------------------------------------------------------------- Todo suprimée --
fn _delete() {

    // ------------------------------------ Récupérer le fichier et retourner le numéro de lignes --

    let file: File = File::open("todo.txt").unwrap(); // Récupérer le fichier
    let out_file: File = File::open("todo.txt.temp").unwrap();

    let read = BufReader::new(&file);
    let mut write = BufWriter::new(&out_file);

    for (_index, line) in read.lines().enumerate() {
        let line = line.as_ref().unwrap();
        if line.contains("Test") {
            writeln!(write, "{}", line);
        } else {
            println!("Erreur");
        }
    }
    fs::rename("todo.txt.temp", "todo.txt").unwrap();

    // println!("Deleting a todo...")

}


// ----------------------------------------------------------------------------------- Todo finie --
fn _done() {
    println!("done")
}


// ------------------------------------------------------------------------------- Todo non finie --
fn _undone() {
    println!("undone")
}


// -------------------------------------------------------------------------------- Todo deadline --
fn _due() {
    println!("due")
}


// ---------------------------------------------------------------------------------- Todo listée --
fn _list() {
    println!("list")
}


// ----------------------------------------------------------------------------------- Todo triée --
fn _sort() {
    println!("sort")
}