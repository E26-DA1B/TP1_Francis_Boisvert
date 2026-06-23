use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone, Copy, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum Statut {
    #[default]
    Disponible,
    Emprunte,
}


impl  Statut {
    pub fn afficher(&self) -> &str {
        match self {
            Statut::Disponible => "\x1b[92mDisponible\x1b[0m",
            Statut::Emprunte => "\x1b[91mEmprunté\x1b[0m",
        }
    }

    pub fn afficher_inverse(&self) -> &str {
        match self {
            Statut::Emprunte => "\x1b[92mDisponible\x1b[0m",
            Statut::Disponible => "\x1b[91mEmprunté\x1b[0m",
        }
    }
}