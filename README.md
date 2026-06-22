# Gestionnaire de Bibliothèque

![Rust Version](https://img.shields.io/badge/rust-2024%2B-orange?style=flat-square&logo=rust)
![Code Status](https://img.shields.io/badge/status-stable%20%2F%20tested-success?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)

Un système de gestion de bibliotheque en mode console développe en Rust. Le programme permet de charger, rechercher, modifier et analyser un catalogue de livres stocke au format JSON, le tout protège par une validation stricte des types et des entrées.

---

## Fonctionnalites

* Chargement Dynamique : Lecture automatique du catalogue depuis un fichier livres.json via Serde.
* Création : Système de modification champ par champ (Titre, Auteur, Annee, Pages, Genre) utilisant une structure temporaire (LivreTemp) pour eviter les données partielles.
* Recherche Avancee : Filtrage  par champ textuel (insensible a la casse) ou par statut (Disponible / Emprunte).
* Gestion des Emprunts : Inversion rapide du statut d'un livre avec confirmation de l'utilisateur.
* Statistiques en Temps Reel : Calcul du nombre total de livres, du total de pages, de la moyenne de pages par livre et de la répartition des statuts (avec sécurité contre la division par zéro).

---

## Architecture du Projet

Le projet est entierement modularise pour separer la logique metier de l'affichage :



src/  
|-- main.rs  
|-- affichage/  
|-- creation_modification/  
|-- livres/  
|-- recherche/  
|-- statistiques/  
`-- utils/  


---

## Installation et Execution

### Prerequis
* Avoir installe la derniere version stable de Rust (cargo).
* Avoir un fichier livres.json valide situe dans le repertoire ../Data/. (Un fichier est fourni en guise d'example)

### Lancement
1. Clonez le depot ou recuperez les fichiers sources.
2. Naviguez dans le dossier contenant le fichier Cargo.toml.
3. Lancez l'application avec la commande suivante :

cargo run

---

## Pistes d'Amelioration

* Implementer une fonction de sauvegarde
* Utiliser un système de gestion d'erreurs plus robuste avec Result et inclure des messages personalisés
* Triage des tableaux par ordre A-Z ou Z-A pour Titre, Auteur, Genre, Triage du tableau par nombre de Pages ou par Année
* Ajouter un système de pagination dans l'affichage du tableau complet si le catalogue contient un grand nombre de volumes.
* Ajouter une fonction pour modifier un livre existant (la structure utilisée pour créer un livre contient toute la logique pour recevoir un livre existant en prevision de l'ajout de cette fonction)
* Ajouter la possibilité de supprimer un livre de la bibliothèque


---

## Qualite du Code

L'application est completement testée et stable. Vous pouvez valider la conformite du code en executant :

cargo check
cargo clippy

## Auteur

Projet réalisé par Francis Boisvert, dans le cadre du TP1 — Rust fondamental.
