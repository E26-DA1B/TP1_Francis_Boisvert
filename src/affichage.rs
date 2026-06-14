use crate::livres::{AfficherStatut, Livre};

pub fn afficher_tableau(livres: &[Livre], titre: &str) {
    println!("{:^92}", titre);
    println!(" ______________________________________________________________________________________");
    println!("| {:<20} | {:<20} | {:<6} | {:<6} | {:<10} | {:<10} |", "Titre", "Auteur", "Année", "Pages", "Genre", "Statut");
    println!("|--------------------------------------------------------------------------------------|");
    for livre in livres {
        println!(
            "| {:<20.20} | {:<20.20} | {:<6} | {:<6} | {:<10.10} | {:<10.10} |",
            livre.titre, livre.auteur, livre.annee, livre.pages, livre.genre, livre.statut.afficher());
    }
    println!("|¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯|");
    println!();
}

pub fn afficher_menu_principal() {
    println!(
        "
+==========================================+
|        Gestionnaire de bibliothèque       |
+==========================================+

  1. Afficher tous les livres
  2. Ajouter un livre
  3. Rechercher un livre par titre
  4. Modifier le statut d'un livre
  5. Afficher les statistiques
  6. Quitter

+==========================================+
"
    );
}

