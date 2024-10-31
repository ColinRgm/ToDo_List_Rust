# Ma ToDo List en Rust

## Fonctionnalités
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

- [ ] Ajoutez un nouveau drapeau --due suivi d'une date au format "YY-MM-DD" afin d'ajouter une deadline.

- [x] Ajoutez un drapeau --list pour afficher toutes les toods et leurs status (done, undone).

- [ ] Ajoutez un drapeau --sort pour trier la liste dans l'ordre de priorités



### Comment utiliser la ToDo List