use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use clap::{command, Arg, ArgAction};
use std::{env, fs};

fn main() {
    delete();
}



// -------------------------------------------------------------- Drapeaux delete et sa fonctions --
fn delete() {


    // -------------------------------------------------------------------- Création des drapeaux --
    let _arguments_delete = command!()
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
            // .conflicts_with() // To avoid conflict between flags
        )
        // Build the instance
        .get_matches();

    // Get the argument
    let args: Vec<String> = env::args().collect();

    // Store the argument
    let num_line = args[2].clone();


    // --------------------------------------------------------- Appel des fonctions des drapeaux --
    if num_line == "1" {
        _function_de_test();
    } else if num_line == "2" {

    }

    // ------------------------------------ Récupérer le fichier et retourner le numéro de lignes --

    let file: File = File::open("todo.txt").expect("Fichier inexistant"); // Récupérer le fichier

    let out_file: File = File::create("todo.txt.temp")
        .expect("Création du fichier impossible"); // Créer un fichier temporaire

    let read = BufReader::new(&file);
    let mut write =  BufWriter::new(&out_file);

    for (index, line) in read.lines().enumerate() {
        let line = line.expect("Erreur de ligne");

        if index != num_line.parse().unwrap() {
            writeln!(write, "{}", line).expect("Erreur");
        }
    }

    fs::rename("todo.txt.temp", "todo.txt").unwrap();


    // ------------------------------------------------------------- Afficher la liste des tâches --
    // println!("Liste des tâches à faire : {}", arguments.get_one::<String>("list").unwrap());
}




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
        .open(path) // Ouvrir le fichier
        .expect("Pas de fichier"); // Message en cas d'erreur


    // --------------------------------------------------------------------- Retourner une erreur --
    if let Err(e) = writeln!(file, "{}", text) {
        eprintln!("Erreur : {}", e); // Si erreur
    } else {
        eprintln!("ToDo ajoutée au fichier: {}", path); // Si tout fonctionne
    }
}





// ----------------------------------------------------------- Fonction servant à faire des tests --
fn _function_de_test() {
    println!("Ceci est un test")
}