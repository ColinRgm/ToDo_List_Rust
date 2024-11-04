# Ma ToDo List en Rust

### Fonctionnalités
- Ajouter de nouvelles todos en une commande

- Supprimer une todo avec son numéro

- Marquer une todo comme terminée

- Ajouter une deadline à une todo

- Afficher toutes les todos

- Afficher par ordre d'urgence

- Sauvegarder les todos dans un fichier JSON



## Étapes
- [x] Créez un nouveau projet Rust, faites un premier commit, poussez le sur GitHub.

- [x] Codez un simple programme qui demande à l'utilisateur d'écrire une todo,
   puis qui l'écrit dans un fichier texte.

- [x] Modifiez le code pour que lorsque vous ajoutez une todo suppémentaire (nouveau lancement du programme),
   une ligne soit ajoutée dans le fichier texte.

- [x] Modifiez le code pour que si l'utilisateur écrit un drapeau --delete suivi du numéro de ligne d'une todo,
   elle soit supprimée du fichier.

- [x] Modifiez le système pour utiliser un fichier JSON pour le stockage des todos, pour ce faire,
   il faudra utiliser une struct rust anisi que la librairie serde.

- [x] Modifiez le code pour utiliser la libraire clap afin de pouvoir gèrer automatiquement
   les arguments et drapeaux de la ligne de commande.

- [x] Ajoutez un drapeau --done suivi du numéro de la todo pour indiquer qu'elle est terminée.

- [x] Ajoutez un drapeau --undone suvi du numéro de la todo pour indiquer qu'elle n'est pas terminée.

- [x] Ajoutez un nouveau drapeau --due suivi d'une date au format "YY-MM-DD" afin d'ajouter une deadline.

- [x] Ajoutez un drapeau --list pour afficher toutes les toods et leurs status (done, undone).

- [x] Ajoutez un drapeau --sort pour trier la liste dans l'ordre de priorités



## Comment utiliser la ToDo List

1. Ouvrir le dossier du projet dans le terminal

2. Lancer la commande cargo build
   3. Permet de compiler le code source et de générer un exécutable

4. Lancer ensuite la commande cargo run
   5. Qui permettra de lancer le projet
   6. Vous demandera également d'ajouter une tâche

7. Si vous voulez supprimer un tâche
   8. cargo run -- --delete --id 1 (supprimera la tâche avec l'ID n°1)

8. Pour marquer une tâche comme terminée
   9. cargo run -- --done --id 1

10. Pour marquer une tâche comme non terminée
    11. cargo run -- --undone --id 1

12. Pour lister vos tâches
    13. cargo run -- --list

14. Pour ajouter une deadline à une tâche
    15. cargo run -- -- due-date "2025-01-01" --id 1

15. Pour trier vos tâches par date (ordre chronologique)
    16. cargo run -- --sort