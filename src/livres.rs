use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Statut {
    Disponible,
    Emprunte,
}

#[derive(Deserialize)]
pub struct Livre {
    pub titre: String,
    pub auteur: String,
    pub annee: i32,
    pub pages: i32,
    pub genre: String,
    pub statut: Statut,
}

pub trait AfficherStatut {
    fn afficher(&self) -> &str;
}

pub trait AfficherLivre {
    fn afficher(&self);
}

impl AfficherStatut for Statut {
    fn afficher(&self) -> &str {
        match self {
            Statut::Disponible => "Disponible",
            Statut::Emprunte => "Emprunté",
        }
    }
}

impl AfficherLivre for Livre {
    fn afficher(&self) {
        println!("Titre: {}", self.titre);
        println!("Auteur: {}", self.auteur);
        println!("Année: {}", self.annee);
        println!("Pages: {}", self.pages);
        println!("Genre: {}", self.genre);
        println!("Statut: {}", self.statut.afficher());
    }
}

pub fn generer_bibliotheque() -> Vec<Livre> {
    let data = std::fs::read_to_string("../Data/livres.json").expect("Impossible de lire le fichier .json");
    serde_json::from_str(&data).expect("JSON invalide dans livres.json")
}
