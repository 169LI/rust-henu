use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Student {
    pub name: String,
    pub id: String,
    pub age: u32,
}