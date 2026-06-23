use serde::Deserialize;
use crate::livres::enum_statut::Statut;


#[derive(Deserialize, Clone, Default, PartialEq)]
#[allow(dead_code)]
#[serde(rename_all = "PascalCase")]
pub struct Livre {
    pub titre: String,
    pub auteur: String,
    #[serde(rename = "Année")]
    pub annee: i32,
    pub pages: i32,
    pub genre: String,
    pub statut: Statut,
}

pub trait Affichable {
    fn afficher(&self);
}

impl Livre {
    pub fn changer_statut(&mut self) {
    self.statut = match self.statut {
        Statut::Emprunte => Statut::Disponible,
        Statut::Disponible => Statut::Emprunte,
        }
    }
}


impl Affichable for Livre {
    fn afficher(&self) {
        println!(" Titre: {}", self.titre);
        println!("Auteur: {}", self.auteur);
        println!(" Année: {}", self.annee);
        println!(" Pages: {}", self.pages);
        println!(" Genre: {}", self.genre);
        println!("Statut: {}", self.statut.afficher());
    }
}

