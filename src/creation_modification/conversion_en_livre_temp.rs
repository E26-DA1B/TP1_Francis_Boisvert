use crate::livres::livre::Livre;
use crate::livres::livre_temp::LivreTemp;

pub fn conversion_en_livre_temp(livre:Option<Livre>) -> LivreTemp {
        match livre {
        Some(l) => LivreTemp {
            titre: Some(l.titre), 
            auteur: Some(l.auteur), 
            annee: Some(l.annee), 
            pages: Some(l.pages), 
            genre: Some(l.genre),
            statut: l.statut,
        },
        None => LivreTemp::default(),
    }

}