use std::fs;
use chrono::NaiveDate;
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::read_to_string;
use std::io::{self};


// ------------------------------------------------------------------------- Récupérer le fichier --
const PATH: &str = "todo.json";


// --------------------------------------------------------------------------- Structure des ToDo --
#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    todo_text: String,
    done: bool,
    due: Option<NaiveDate>,
}


// ------------------------------------------------------------------------ Création des drapeaux --
#[derive(Parser, Debug)]
#[command(version, about = "Your Todo")]
struct Flags {
    /// Delete a todo
    #[arg(long)]
    delete: bool, // Delete argument

    /// Mark a todo as done
    #[arg(long)]
    done: bool, // Done argument

    /// Mark a todo as undone
    #[arg(long)]
    undone: bool, // Undone argument

    /// Add a date for the todo
    #[arg(long)]
    due_date: Option<String>, // Deadline argument

    /// List the todo
    #[arg(long)]
    list: bool, // List argument

    /// Sort the todo
    #[arg(long)]
    sort: bool, // Sort argument

    #[arg(long, default_value_t = 0)]
    id: usize, // ID ToDo
}


fn main() -> io::Result<()> {
    let flag = Flags::parse();


    // ------------------------------------------------------------------------ Ouvrir le fichier --
    let mut todos: Vec<Todo> = match read_to_string("todo.json") {
        Err(_) => Vec::new(),
        Ok(todo_list) => {
            if todo_list.trim().is_empty() {
                Vec::new() // Si le fichier est vide, Vec::new
            } else {
                serde_json::from_str(&todo_list).expect("Analyse du fichier impossible")
            }
        }
    };


    // Call the "delete" function if delete is in agrument -----------------------------------------
    if flag.delete
    {
        // Nécessite l'ID de la ToDO -> flag.id
        delete(&mut todos, flag.id)
    }
    // Call the "done" function if done is in agrument ---------------------------------------------
    else if flag.done
    {
        // Nécessite l'ID de la ToDO -> flag.id
        done(&mut todos, flag.id)
    }
    // Call the "undone" function if undone is in agrument -----------------------------------------
    else if flag.undone
    {
        // Nécessite l'ID de la ToDO -> flag.id
        undone(&mut todos, flag.id)
    }
    // Call the "list" function if list is in agrument ---------------------------------------------
    else if flag.list
    {
        list(&mut todos)
    }
    // Call the "sort" function if sort is in agrument ---------------------------------------------
    else if flag.sort
    {
        sort(&mut todos)
    }
    // Call the "due" function if due is in argument -----------------------------------------------
    else if let Some(due_date) = flag.due_date
    {
        // Nécessite l'ID et la date de fin de la ToDO -> flag.id, &due_date
        due(&mut todos, flag.id, &due_date)
    }
    // Call the "add" function if nothing is in agrument -------------------------------------------
    else {
        add(&mut todos)
    }


    // Écriture dans le fichier JSON (reformaté)
    fs::write(PATH, serde_json::to_string_pretty(&todos)
        .expect("Sérialisation impossible"))
        .expect("Écriture impossible");

    Ok(())
}


// --------------------------------------------------------------------------------- Todo ajoutée --
fn add(todos: &mut Vec<Todo>) {

    // println!("Add")

    println!("ToDo à ajouter :");

    // Lecture de l'entrée utilisateur
    let mut text = String::new(); // Variable mutable qui stocke l'entrée de l'utilisateur

    io::stdin().read_line(&mut text)
        .expect("Lecture de ligne impossible");

    let users_todo = Todo {
        todo_text: text.trim().to_string(),
        done: false,
        due: None,
    };

    todos.push(users_todo)
}


// -------------------------------------------------------------------------------- Todo suprimée --
fn delete(todos: &mut Vec<Todo>, id: usize) {

    // println!("Delete");

    // Si l'ID et plus grand que 0 mais plus petit que la longueur de la liste
    if id > 0 && id <= todos.len()
    {
        todos.remove(id - 1);

        println!("ToDo supprimée")
    }
    // Si l'ID n'est pas compris entre 0 et la longeur de la liste
    else {
        println!("ID invalide")
    }
}


// ----------------------------------------------------------------------------------- Todo finie --
fn done(todos: &mut Vec<Todo>, id: usize) {

    // println!("Done");

    // Si l'ID et plus grand que 0 mais plus petit que la longueur de la liste
    if id > 0 && id <= todos.len()
    {
        todos[id - 1].done = true;
        println!("ToDo terminée")
    }
    // Si l'ID n'est pas compris entre 0 et la longeur de la liste
    else {
        println!("ID invalide")
    }
}


// ------------------------------------------------------------------------------- Todo non finie --
fn undone(todos: &mut Vec<Todo>, id: usize) {

    // println!("Undone");

    // Si l'ID et plus grand que 0 mais plus petit que la longueur de la liste
    if id > 0 && id <= todos.len()
    {
        todos[id - 1].done = false;
        println!("ToDo non-terminée")
    }
    // Si l'ID n'est pas compris entre 0 et la longeur de la liste
    else {
        println!("ID invalide")
    }
}


// -------------------------------------------------------------------------------- Todo deadline --
fn due(_todos: &mut Vec<Todo>, _id: usize, _due_date: &str) {

    println!("Due");
}


// ---------------------------------------------------------------------------------- Todo listée --
fn list(todos: &mut Vec<Todo>) {

    // println!("List");

    if todos.is_empty()
    {
        println!("Aucune tâche à afficher.");
    }
        // Afficher les todos avec leurs status
    else
    {
        for (i, todo) in todos.iter().enumerate() {
            let status = if todo.done
            {
                "Terminée"
            } else {
                "Non terminée"
            };
            println!("{}: [{}] {}", i + 1, status, todo.todo_text);
        }
    }
}


// ----------------------------------------------------------------------------------- Todo triée --
fn sort(todos: &mut Vec<Todo>) {

    // println!("Sort");

    todos.sort_by(|a, b| a.due.cmp(&b.due));

}