use crate::livres::bibliotheque::Bibliotheque;
use crate::livres::livre::Livre;

pub fn sauvegarder_livre(livre:Livre, bibliotheque: &mut Bibliotheque) {
    bibliotheque.livres.push(livre);
}