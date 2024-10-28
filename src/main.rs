use std::fs::OpenOptions;
use std::io::Write;
use clap::{Parser};

// ----------------------------------------------------------- Fonction servant à faire des tests --
fn _function_de_test() {
    println!("Ceci est un test")
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Flags {
    /// Delete a todo
    #[arg(long)]
    delete: bool,

    /// Mark a todo as done
    #[arg(long)]
    done: bool,

    /// Mark a todo as undone
    #[arg(long)]
    undone: bool,

    /// Add a deadline for the todo
    #[arg(long)]
    due: bool,

    /// List the todo
    #[arg(long)]
    list: bool,

    /// Sort the todo
    #[arg(long)]
    sort: bool,

    #[arg(required: false)]
    id: u8

}

fn main() {

    let args = Flags::parse();

    if args.delete {
        _delete() // Call the delete function if delete is in agrument
    } else if args.done {
        _done() // Call the done function if done is in agrument
    } else if args.undone {
        _undone() // Call the undone function if undone is in agrument
    } else if args.due {
        _due() // Call the due function if due is in agrument
    } else if args.list {
        _list() // Call the due function if list is in agrument
    } else if args.sort {
        _sort() // Call the sort function if sort is in agrument
    } else {
        _add() // Call the add function if nothing is in agrument
    }
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
    if let Err(e) = writeln!(file, "\n{}", text) {
        eprintln!("Erreur : {}", e); // Si erreur
    } else {
        eprintln!("ToDo ajoutée au fichier: {}", path); // Si tout fonctionne
    }
}




// -------------------------------------------------------------------------------- Todo suprimée --
fn _delete() {
    println!("delete a todo");
}




// ----------------------------------------------------------------------------------- Todo finie --
fn _done() {
    println!("done");
}




// ------------------------------------------------------------------------------- Todo non finie --
fn _undone() {
    println!("undone");
}




// -------------------------------------------------------------------------------- Todo deadline --
fn _due() {
    println!("due");
}




// ---------------------------------------------------------------------------------- Todo listée --
fn _list() {
    println!("list");
}




// ----------------------------------------------------------------------------------- Todo triée --
fn _sort() {
    println!("sort");
}