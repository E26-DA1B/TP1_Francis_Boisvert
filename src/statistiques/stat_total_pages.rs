use crate::livres::bibliotheque::Bibliotheque;


pub fn stat_total_pages(bibliotheque: &Bibliotheque) -> u32{
    bibliotheque.livres.iter().map(|l| l.pages).sum()
}