use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use clap::{Command, Arg, ArgAction};
use std::{fs};


// ----------------------------------------------------------- Fonction servant à faire des tests --
fn _function_de_test() {
    println!("Ceci est un test")
}

fn main() {

    // -------------------------------------------------------------------- Creation des drapeaux --
    let arguments = Command::new("myflags")
        .about("Welcome to your personnal ToDo List")
        .arg
        (
            // Add flag
            Arg::new("add")
                .short('a') // Short Name
                .long("add") // Long Name
                .help("Add a todo") // Description
                .action(ArgAction::SetTrue)
        ).arg
    (
        // Delete flag
        Arg::new("delete")
            .short('d') // Short Name
            .long("delete") // Long Name
            .help("Delete a todo") // Description
            .action(ArgAction::SetTrue)
    ).arg
    (
        // Done flag
        Arg::new("done")
            .short('o') // Short Name
            .long("done") // Long Name
            .help("Mark a todo done") // Description
            .action(ArgAction::SetTrue)
    ).arg
    (
        // Undone flag
        Arg::new("undone")
            .short('u') // Short Name
            .long("undone") // Long Name
            .help("Mark a todo undone") // Description
            .action(ArgAction::SetTrue)
    ).arg
    (
        // Due flag
        Arg::new("due")
            .short('e') // Short Name
            .long("due") // Long Name
            .help("Todo to do") // Description
            .action(ArgAction::SetTrue)
    ).arg
    (
        // List flag
        Arg::new("list")
            .short('l') // Short Name
            .long("list") // Long Name
            .help("List the todo") // Description
            .action(ArgAction::SetTrue)
    ).arg
    (
        Arg::new("sort")
            .short('s') // Short Name
            .long("sort") // Long Name
            .help("Sort the todo") // Description
            .action(ArgAction::SetTrue)
    )
        .get_matches(); // Build the instance


    // Call the right arg at the right time
    if arguments.contains_id("add") {
        _add();
    } else if arguments.contains_id("delete") {
        _delete();
    } else if arguments.contains_id("done") {
        _done();
    } else if arguments.contains_id("undone") {
        _undone();
    } else if arguments.contains_id("due") {
        _due();
    } else if arguments.contains_id("list") {
        _list();
    } else if arguments.contains_id("sort") {
        _sort();
    }

    // Get the argument
    // let args: Vec<String> = env::args().collect();

    // Store the argument
    /* if args.len() > 2 {
        let input = args[1].clone();
        println!("{}", input);
    }
    */
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
    let file: File = File::open("todo.txt")
        .expect("Impossible d'ouvrir le fichier"); // Ouvrir le fichier

    let out_file: File = File::open("todo.txt.temp")
        .expect("Création impossible"); // Créer une fichier temporaire

    let read = BufReader::new(file);
    let mut write = BufWriter::new(out_file);

    for line in read.lines() {
        let line = line.
            expect("Erreur");

        if !line.contains("Test") {
            writeln!(write, "{}", line)
                .expect("Erreur")
        }
    }

    fs::rename("todo.txt.temp", "todo.txt")
        .expect("Impossible de renommer le fichier");

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