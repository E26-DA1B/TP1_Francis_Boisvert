mod creation_modification;
mod utils;
mod livres;
mod affichage;
mod recherche;


// use menu::creation_livre;




use utils::validation;
use validation::valider_choix;

use creation_modification::menu_creation_modification_livre::creation_livre;

use affichage::affichage_menu_principal::afficher_menu_principal;
use affichage::affichage_tableau::afficher_tableau;


// use crate::affichage::affichage_menu_recherche_champ::menu_recherche_par_champs;
use crate::livres::bibliotheque::Bibliotheque;
use crate::recherche::menu_rechercher::rechercher;
use crate::creation_modification::menu_changer_statut::menu_changer_statut;


fn main() {
    let mut bibliotheque = Bibliotheque::charger();

    loop {
        afficher_menu_principal();
        let choix_utilisateur = valider_choix(6);
        match choix_utilisateur {
            Some(1) => { afficher_tableau(&bibliotheque.livres, "\nListe complete de la bibliotheque"); },
            Some(2) => { creation_livre(None, &mut bibliotheque) },
            Some(3) => { rechercher(&bibliotheque); },
            Some(4) => { menu_changer_statut(&mut bibliotheque) },
            Some(5) => {},
            Some(6) => {break},
            _ => {},
        }
    }
    
}
