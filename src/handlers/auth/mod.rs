use serde::Deserialize;

pub mod discord;
pub mod spotify;

#[derive(Debug, Deserialize)]
pub struct CallbackQuery {
    code: String,
    state: String,
}
