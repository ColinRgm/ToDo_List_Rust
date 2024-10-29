use std::fs::{File, OpenOptions};
use std::io;
use clap::{Parser};
use std::io::{BufRead, BufReader, Write};



// ----------------------------------------------------------- Fonction servant à faire des tests --
fn _function_de_test() {
    println!("Ceci est un test")
}



// ------------------------------------------------------------------------- Récupérer le fichier --
const PATH: &str = "todo.txt";



// ------------------------------------------------------------------------- Récupérer les lignes --
fn get_line() {
    // Lister les entrées du fichier
    let file = File::open(PATH).expect("Pas de fichier");

    let reader = BufReader::new(file);

    for (line_num, line) in reader.lines().enumerate() {
        let line = line.expect("Impossible de lire la ligne");

        let _id = line_num + 1;

        println!("{}", line);
    }
}




// ------------------------------------------------------------------------ Création des drapeaux --
#[derive(Parser, Debug)]
#[command(version, about = "Welcome on your personnal Todo list")]
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


    /// ID of the line
    #[arg(long)]
    id: Option<u8>,
}

fn main() {

    let args = Flags::parse();

    if args.delete
    {
        delete(); // Call the "delete" function if delete is in agrument

        if args.id > Some(0) {
            println!("Ligne choisie : {:?}", args.id);
        }

    } else if args.done
    {
        done() // Call the "done" function if done is in agrument

    } else if args.undone
    {
        undone() // Call the "undone" function if undone is in agrument

    } else if args.due
    {
        due() // Call the "due" function if due is in agrument

    } else if args.list
    {
        list() // Call the "list" function if list is in agrument

    } else if args.sort
    {
        sort() // Call the "sort" function if sort is in agrument

    } else
    {
        add() // Call the "add" function if nothing is in agrument

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
    println!("Delete a todo");

    get_line(); // Appelle de la fonction de listing avec le numéro de chaque lignes

    // Effacer la ligne à la ligne entrée en argument

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