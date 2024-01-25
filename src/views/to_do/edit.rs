use crate::json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems};
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::{enums::TaskStatus, to_do_factory};
use actix_web::{web, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json");
    let status = match &state.get(&to_do_item.title) {
        Some(result) => TaskStatus::new(result.as_str().unwrap()),
        None => {
            return HttpResponse::NotFound().json(format!("{} not in state", &to_do_item.title))
        }
    };
    HttpResponse::Ok().json(ToDoItems::get_state())
}
