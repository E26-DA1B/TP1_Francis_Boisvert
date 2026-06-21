use crate::creation_modification::modifier_champ::modifier_champ;

use crate::livres::champ::Champ;
use crate::livres::livre::Livre;

use crate::creation_modification::conversion_en_livre_complet::conversion_livre_complet;
use crate::creation_modification::conversion_en_livre_temp::conversion_en_livre_temp;

use crate::affichage::affichage_couleurs_messages::afficher_message_jaune;




pub fn changer_statut_livre(livre: &mut Livre) {
    let old = livre.clone();
    let mut livre_temp = conversion_en_livre_temp(Some(std::mem::take(livre)));
    modifier_champ(&mut livre_temp, (Champ::Statut, "".to_string()));
    *livre = match conversion_livre_complet(&livre_temp){
        Ok(l) => l, 
        Err(e) => {
            afficher_message_jaune(&e);
            old
        }
    };
}