# Comment utiliser la ToDo List

1. Récupérer le dossier sur GitHub
   2. ```shell
      git clone git@github.com:ColinRgm/ToDo_List_Rust.git

1. Ouvrir le dossier du projet dans le terminal
   2. ```shell
      cd ToDo_List_Rust

2. Lancer la commande cargo build
   3. Permet de compiler le code source et de générer un exécutable

4. Lancer ensuite la commande cargo run
   5. Qui permettra de lancer le projet
   6. Vous demandera également d'ajouter une tâche

7. Si vous voulez supprimer un tâche (supprimera la tâche avec l'ID n°1)
   8. ```shell
      cargo run -- --delete --id 1

8. Pour marquer une tâche comme terminée
   9. ```shell
      cargo run -- --done --id 1

10. Pour marquer une tâche comme non terminée
    11. ```shell
        cargo run -- --undone --id 1

12. Pour lister vos tâches
    13. ```shell
        cargo run -- --list

14. Pour ajouter une deadline à une tâche
    15. ```shell
        cargo run -- -- due-date "2025-01-01" --id 1

15. Pour trier vos tâches par date (ordre chronologique)
    16. ```shell
        cargo run -- --sort
