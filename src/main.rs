use std::string::String;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use clap::{command, Arg, ArgAction, Command};
use std::env;

fn main() {
    flags();
}


// ------------------------------------------------------------------ Drapeaux et leurs fonctions --
fn flags() {


    // -------------------------------------------------------------------- Création des drapeaux --
    /*let arguments = command!()
        .about("Welcome to your personnal ToDo list ! \n
                PLease do not delete line 0")
        .arg
        (
            // Delete flag
            Arg::new("delete")
                .short('d')// Short Name
                .long("delete")// Long Name
                .help("To delete a todo")// Description
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
                .action(ArgAction::SetTrue)
                .conflicts_with("delete")
        )
        // Build the instance
        .get_matches();
        */

    let arguments = Command::new("My ToDo")
        .arg(
            Arg::new("delete")
                .short('d')
                .long("delete")
                .action(ArgAction::Append)
        );

    let num = arguments.try_get_matches_from(["My ToDo", "-d", "value1", "--delete", "value2"]).unwrap();
    assert!(matches.contains_id("delete"));
    assert_eq!(
        matches.get_many::<String>("delete").unwrap_or_default().map(|n| n.as_str()).collect::<Vec<_>>(),
        vec!["value1", "value2"]
    )

    /*
    if let Some(config_file) = matches!(get_one::<String>("delete")) {
        println!("Value for config: {}", config_file);
    } else {
        println!("No config file specified");
    }
    */


    // --------------------------------------------------------- Appel des fonctions des drapeaux --
    if arguments.get_flag("delete")  {
        test_for_arguments();
        delete_todo();
    } else if arguments.get_flag("add") {
        // test_for_arguments();
        add_todo();
    }


    // ------------------------------------------------------------- Afficher la liste des tâches --
    // println!("Liste des tâches à faire : {}", arguments.get_one::<String>("list").unwrap());
}

fn test_for_arguments() {
    println!("Tout fonctionne !")
}


// --------------------------------------------------------------------------------- Ajouter todo --
fn add_todo() {
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
fn delete_todo() {
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