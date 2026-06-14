use crate::livres::Livre;

pub fn afficher_livre(l: &Livre) {
    println!("Titre: {}", l.titre);
    println!("Auteur: {}", l.auteur);
    println!("Année: {}", l.annee);
    println!("Pages: {}", l.pages);
    println!("Genre: {}", l.genre);
    println!("Statut: {}", l.statut);
}

pub fn afficher_bibliotheque(livres: &[Livre]) {
    println!("Liste complete des livres");
    println!();
    for l in livres {
        afficher_livre(l);
        println!();
    }
    println!();
    println!("Fin de la Liste");
}



pub fn afficher_bibliotheque_tableau(livres: &[Livre]) {
    println!(" ______________________________________________________________________________________");
    println!("| {:<20} | {:<20} | {:<6} | {:<6} | {:<10} | {:<10} |",
        "Titre", "Auteur", "Année", "Pages", "Genre", "Statut");
    println!("|--------------------------------------------------------------------------------------|");
    for livre in livres {
        println!(
            "| {:<20.20} | {:<20.20} | {:<6} | {:<6} | {:<10.10} | {:<10.10} |",
            livre.titre,
            livre.auteur,
            livre.annee,
            livre.pages,
            livre.genre,
            livre.statut
        );
    }
    println!("|______________________________________________________________________________________|");
    println!();
}
