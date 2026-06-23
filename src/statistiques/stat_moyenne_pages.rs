use crate::livres::bibliotheque::Bibliotheque;


pub fn stat_moyenne_pages(bibliotheque: &Bibliotheque) -> f64{
    let total_livres = bibliotheque.livres.len() as f64;
    if total_livres == 0.0 {
        return 0.0
    }
    let total_pages: f64  = bibliotheque.livres.iter().map(|l| l.pages).sum::<u32>() as f64;

    total_pages / total_livres
    
}