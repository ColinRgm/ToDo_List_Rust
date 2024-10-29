use std::fs::{File, OpenOptions};
use clap::Parser;
use std::io::{BufRead, BufReader, Write};
use std::io;


// ------------------------------------------------------------------------- Récupérer le fichier --
const PATH: &str = "todo.txt";


// --------------------------------------------------------------- Récupérer les numéros de ligne --
fn get_line() {

    // Lister les entrées du fichier
    let file = File::open(PATH).expect("Pas de fichier");

    let reader = BufReader::new(file);

    for (line_num, line) in reader.lines().enumerate() {
        let line = line.expect("Impossible de lire la ligne");

        let id = line_num + 1;

        println!("{}, content -> {}", id, line);
    }
}


// ------------------------------------------------------------------------ Création des drapeaux --
#[derive(Parser, Debug)]
#[command(version, about = "Welcome on your personnal Todo list")]
struct Flags {
    /// Delete a todo
    #[arg(long)]
    delete: Option<usize>, // Delete argument

    /// Mark a todo as done
    #[arg(long)]
    done: Option<usize>, // Done argument

    /// Mark a todo as undone
    #[arg(long)]
    undone: Option<usize>, // Undone argument

    /// Add a date for the todo
    #[arg(long)]
    due_date: Option<usize>, // Deadline argument

    /// Index for the due arg
    #[arg(long)]
    due_index: Option<usize>, // Index argument

    /// List the todo
    #[arg(long)]
    list: bool, // List argument

    /// Sort the todo
    #[arg(long)]
    sort: bool, // Sort argument

    /// Add a todo
    #[arg(long)]
    add: Option<usize>, // Add argument

}

#[derive(Debug)]
struct Todo {
    text: String,
    done: bool,
    due_date: Option<String>,
}


fn main() {
    let flag = Flags::parse();

    // Call the "delete" function if delete is in agrument -----------------------------------------
    if flag.delete
    {
        delete();
    }
    // Call the "done" function if done is in agrument ---------------------------------------------
    else if flag.done
    {
        done()
    }
    // Call the "undone" function if undone is in agrument -----------------------------------------
    else if flag.undone
    {
        undone()
    }
    // Call the "list" function if list is in agrument ---------------------------------------------
    else if flag.list
    {
        list()
    }
    // Call the "sort" function if sort is in agrument ---------------------------------------------
    else if flag.sort
    {
        sort()
    }
    // Call the "add" function if nothing is in agrument -------------------------------------------
    else {
        add()
    }
}


// --------------------------------------------------------------------------------- Todo ajoutée --
fn add() {
    println!("ToDo à ajouter :");

    // Lecture de l'entrée utilisateur
    let mut text = String::new(); // Variable mutable qui stocke l'entrée de l'utilisateur

    io::stdin()
        .read_line(&mut text)
        .expect("Erreur"); // Lire l'entrée

    let text = text.trim();

    // Ajout du texte dans le fichier txt
    let mut file = OpenOptions::new()
        .append(true) // Ajout du texte
        .create(true) // Créer un fichier si il n'existe pas
        .open(PATH) // Ouvrir le fichier
        .expect("Pas de fichier"); // Message en cas d'erreur

    if let Err(e) = writeln!(file, "{}", text) {
        eprintln!("Woops: {}", e);
    } else {
        eprintln!("ToDo ajoutée !");
    }
}


// -------------------------------------------------------------------------------- Todo suprimée --
fn delete() {
    println!("Delete");

    // get_line();

}


// ----------------------------------------------------------------------------------- Todo finie --
fn done() {
    println!("Done");

    get_line(); // Appelle de la fonction de listing avec le numéro de chaque lignes

    // Fermer la tâche à la ligne entrée en argument

}


// ------------------------------------------------------------------------------- Todo non finie --
fn undone() {
    println!("Undone");

    get_line(); // Appelle de la fonction de listing avec le numéro de chaque lignes

    // Réouvrir la tâche à la ligne entrée en argument

}


// -------------------------------------------------------------------------------- Todo deadline --
fn due() {
    println!("Due");

    // Récupérer le numéro de la ligne (utilisable en tant qu'ID)
    // Ajouter une deadline à la ligne entrée en argument

}


// ---------------------------------------------------------------------------------- Todo listée --
fn list() {
    // println!("List");

    get_line(); // Appelle de la fonction de listing avec le numéro de chaque lignes

    // Récupérer les valeurs done et undone

}


// ----------------------------------------------------------------------------------- Todo triée --
fn sort() {
    println!("Sort");

    // Trier les valeurs du fichier par ordre de priorité

}