use crate::livres::bibliotheque::Bibliotheque;
use crate::livres::enum_statut::Statut;

pub fn stat_emprunte(bibliotheque: &Bibliotheque) -> u32{
    bibliotheque.livres.iter().filter(|l| l.statut == Statut::Emprunte).count() as u32
    
}