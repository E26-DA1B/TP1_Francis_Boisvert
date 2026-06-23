use crate::livres::enum_statut::Statut;


#[allow(dead_code)]
#[derive(Default)]
pub struct LivreTemp {
    pub titre: Option<String>,
    pub auteur: Option<String>,
    pub annee: Option<i32>,
    pub pages: Option<i32>,
    pub genre: Option<String>,
    pub statut: Statut,
}

impl LivreTemp {
    pub fn est_complet(&self) -> bool {
        self.titre.is_some()
        && self.auteur.is_some()
        && self.annee.is_some()
        && self.pages.is_some()
        && self.genre.is_some()
    }
}