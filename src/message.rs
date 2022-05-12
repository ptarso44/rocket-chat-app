use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    #[field(validate = len (..30))]
    pub room: String,
    #[field(validate = len (..20))]
    pub username: String,
    #[field(validate = len (..1000))]
    pub message: String,
}
