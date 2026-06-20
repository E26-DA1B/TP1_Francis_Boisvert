use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub enum Statut {
    #[default]
    Disponible,
    Emprunte,
}