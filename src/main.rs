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


use crate::livres::bibliotheque::Bibliotheque;


fn main() {
    let mut bibliotheque = Bibliotheque::charger();

    loop {
        afficher_menu_principal();
        let choix_utilisateur = valider_choix(6);
        match choix_utilisateur {
            Some(1) => {afficher_tableau(&bibliotheque.livres, "\nListe complete de la bibliotheque");},
            Some(2) => { creation_livre(None, &mut bibliotheque) },
            Some(3) => {},
            Some(4) => {},
            Some(5) => {},
            Some(6) => {break},
            _ => {},
        }
    }
    
}
