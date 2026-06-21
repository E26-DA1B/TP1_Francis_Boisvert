

pub fn affichage_statistiques(total_livres: i32, total_pages: i32, moyenne_pages: f64, nombre_disponible: i32, nombre_emprunte: i32) {


    println!();
    println!("{:^42}","+==========================================+");
    println!("{:^42}","|      Statistiques de la bibliothèque     |");
    println!("{:^42}","+==========================================+");
    println!();
    print!("{:>31}", format!(" Nombre total de livres:"));
    println!("{:>10}", total_livres);

    print!("{:>31}", format!(" Nombre total de pages:" ));
    println!("{:>10}", total_pages);

    print!("{:>31}", format!(" Moyenne de pages par livre:" ));
    println!("{:>10.2}", moyenne_pages);


    println!();
    println!("{:^42}","Nombre disponible    Nombre empruntes");
    println!("{:^21}{:^21}", nombre_disponible, nombre_emprunte);
    println!();
    println!("+==========================================+");
}
