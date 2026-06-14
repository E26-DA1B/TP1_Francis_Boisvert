use serde::Deserialize;

#[derive(Deserialize)]
pub struct Livre {
    pub titre: String,
    pub auteur: String,
    pub annee: i32,
    pub pages: i32,
    pub genre: String,
    pub statut: String,
}


pub fn generer_bibliotheque() -> Vec<Livre> {
    let data = std::fs::read_to_string("../Data/livres.json").expect("Impossible de lire le fichier .json");
    serde_json::from_str(&data).expect("JSON invalide dans livres.json")
}
