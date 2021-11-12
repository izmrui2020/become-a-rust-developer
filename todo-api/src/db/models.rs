use super::schema::todos;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Todo {
    pub id:         i32,
    pub kind:       String,
    pub contents:   String,
    pub done:       bool
}