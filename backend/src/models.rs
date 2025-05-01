use diesel::prelude::*;
use crate::schema::todos;
use serde::{Serialize, Deserialize};
#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::todos)]
pub struct Todo {
    pub id: i32,
    pub date: String,
    pub inhalt: String,
    pub percent: i32,
}



#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub date: &'a str,
    pub inhalt: &'a str,
    pub percent: i32,
}
