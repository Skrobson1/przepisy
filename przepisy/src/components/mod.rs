use serde::{Deserialize, Serialize};
pub mod login;
pub mod page;
pub mod recipe_card;
pub mod recipe_detail;
pub mod settings;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserSettings {
    pub theme: String,
    pub lang: String,
}
