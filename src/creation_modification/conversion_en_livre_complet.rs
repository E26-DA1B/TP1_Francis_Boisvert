use crate::livres::livre::Livre;
use crate::livres::livre_temp::LivreTemp;


pub fn conversion_livre_complet(livre: &LivreTemp) ->  Result<Livre, String>{
    if !livre.est_complet() {
        return Err("Le livre est incomplet, veuillez remplir tous les champs.".into());
    }

    Ok(Livre {
        titre: livre.titre.clone().unwrap(),
        auteur: livre.auteur.clone().unwrap(),
        annee: livre.annee.unwrap(),
        pages: livre.pages.unwrap(),
        genre: livre.genre.clone().unwrap(),
        statut: livre.statut,
    })

}
