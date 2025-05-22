use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Student {
    pub name: String,
    pub id: String,
    pub age: u32,
}