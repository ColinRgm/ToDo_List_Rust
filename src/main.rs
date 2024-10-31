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
    text: String,
    done: bool,
    deadline: Option<NaiveDate>,
}


// ------------------------------------------------------------------------ Création des drapeaux --
#[derive(Parser, Debug)]
#[command(version, about = "Welcome on your personnal Todo list")]
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

    #[arg(long, default_value_t = 0)]
    id: usize, // ID ToDo
}


fn main() -> std::io::Result<()> {
    let mut flag = Flags::parse();


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
        delete(&mut todos, flag.id)
    }
    // Call the "done" function if done is in agrument ---------------------------------------------
    else if flag.done
    {
        done(&mut todos, flag.id)
    }
    // Call the "undone" function if undone is in agrument -----------------------------------------
    else if flag.undone
    {
        undone(&mut todos, flag.id)
    }
    // Call the "list" function if list is in agrument ---------------------------------------------
    else if flag.list
    {
        list(&todos)
    }
    // Call the "sort" function if sort is in agrument ---------------------------------------------
    else if flag.sort
    {
        sort(&mut todos)
    }
    // Call the "add" function if nothing is in agrument -------------------------------------------
    else
    {
        add(&mut todos)
    }

    Ok(())
}


// --------------------------------------------------------------------------------- Todo ajoutée --
fn add(todos: &mut Vec<Todo>) {
    println!("ToDo à ajouter :");

    // Lecture de l'entrée utilisateur
    let mut text = String::new(); // Variable mutable qui stocke l'entrée de l'utilisateur

    io::stdin().read_line(&mut text).expect("Lecture de ligne impossible");

    let users_todo = Todo {
        message: text.trim().to_string(),
        status: false,
        deadline: None,
    }

    todos.push(users_todo)
}


// -------------------------------------------------------------------------------- Todo suprimée --
fn delete(todos: &mut Vec<Todo>, id: usize) {
    if id > 0 && id <= todos.len() {
        todos.remove(id - 1);

        println!("ToDo supprimée")
    }
    else {
        println!("ID invalide")
    }
}


// ----------------------------------------------------------------------------------- Todo finie --
fn done() {
    println!("Done");
}


// ------------------------------------------------------------------------------- Todo non finie --
fn undone() {
    println!("Undone");
}


// -------------------------------------------------------------------------------- Todo deadline --
fn due() {
    println!("Due");
}


// ---------------------------------------------------------------------------------- Todo listée --
fn list() {
    println!("List");
}


// ----------------------------------------------------------------------------------- Todo triée --
fn sort() {
    println!("Sort");
}