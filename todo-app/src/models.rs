use serde::{Deserialize, Serialize};
use super::schema::todos;


#[derive(Queryable, Serialize)]
pub struct Todo {
    pub id: String,
    pub kind: String,
    pub contents: String,
    pub done: bool
}  
