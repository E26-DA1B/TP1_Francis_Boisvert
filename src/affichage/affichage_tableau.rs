use crate::livres::livre::{AfficherStatut, Livre};


pub fn afficher_tableau(livres: &[Livre], titre: &str) {
    println!("{:^92}", titre);
    println!(" _________________________________________________________________________________________");
    println!("| {:<20} | {:<20} | {:<6} | {:<6} | {:<10} | {:<10} |", "Titre", "Auteur", "Année", "Pages", "Genre", "Statut");
    println!("|-----------------------------------------------------------------------------------------|");
    for livre in livres {
        println!(
            "| {:<20.20} | {:<20.20} | {:<6} | {:<6} | {:<10.10} | {:19.19} |",
            livre.titre, livre.auteur, livre.annee, livre.pages, livre.genre, livre.statut.afficher());
    }
    println!(" ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯");
    println!();
}