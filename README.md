# Ma ToDo List en Rust

## Fonctionnalités
- Ajouter de nouvelles todos en une commande
- Supprimer une todo avec son numéro
- Marquer une todo comme terminée
- Ajouter une deadline à une todo
- Afficher toutes les todo
- Afficher par ordre d'urgence
- Sauvegarder les todo dans un fichier JSON

## Étapes
- [ ] Créez un nouveau projet Rust, faites un premier commit, poussez le sur GitHub.
- [ ] Codez un simple programme qui demande a l'utilisateur d'écrire une todo,
   puis qui l'écrire dans un fichier texte.
- [ ] Modifiez le code pour que lorsque vous ajoutez une todo suppémentaire (nouveau lancement du programme),
   une ligne soit ajoutée dans le fichier texte.
- [ ] Modifiez le code pour que si l'utilisateur écris un drapeau --delete suivi du numéro de linge d'une todo,
   elle soit supprimée du fichier.
- [ ] Modifiez le système pour utiliser un fichier JSON pour le stockage des todos, pour ce faire,
   il faudra utiliser une struct rust anisi que la librairie serde.
- [ ] Modifiez le code pour utiliser la libraire clap afin de pouvoir gèrer automatiquement
   les arguments et drapeaux de la ligne de commande.
- [ ] Ajoutez un drapeau --done suivi du numéro de la todo pour indiquer qu'elle est terminée.
- [ ] Ajoutez un drapeau --undone suvi du numéro de la todo pour indiquer qu'elle n'est pas terminée.
- [ ] Ajoutez un nouveau drapau --due suivi d'une date au format "YY-MM-DD" afin d'ajouter une deadline.
- [ ] Ajoutez un drapeau --list pour afficher toute les toods et leur status (done, undone).
- [ ] Ajoutez un trapeau --sort pour trier la liste dans l'ordre de priorités

### Comment utiliser la ToDo List