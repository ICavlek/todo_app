use serde::Serialize;

#[derive(Serialize)]
pub struct ToDoItem {
    pub title: String,
    pub status: String,
}
