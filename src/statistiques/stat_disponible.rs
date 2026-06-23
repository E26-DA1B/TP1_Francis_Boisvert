use crate::livres::bibliotheque::Bibliotheque;
use crate::livres::enum_statut::Statut;

pub fn stat_disponible(bibliotheque: &Bibliotheque) -> u32{
    bibliotheque.livres.iter().filter(|l| l.statut == Statut::Disponible).count() as u32
    
}