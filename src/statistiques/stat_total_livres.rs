use crate::livres::bibliotheque::Bibliotheque;

pub fn stat_nombre_livres(bibliotheque: &Bibliotheque) -> u32 {
    bibliotheque.livres.len() as u32
}