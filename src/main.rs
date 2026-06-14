mod livres;
mod affichage;

use livres::generer_bibliotheque;
use affichage::{afficher_menu_principal, afficher_tableau};


fn main() {
    let mut bibliotheque = generer_bibliotheque();
    afficher_menu_principal();
    // let mut choix_utilisateur ;
    
    
}
