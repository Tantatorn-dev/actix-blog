use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}
