use actix_web::{web, Responder};
use serde_json::value::Value;
use serde_json::Map;

use crate::json_serialization::to_do_items::ToDoItems;
use crate::state::read_file;
use crate::to_do::enums::TaskStatus;
use crate::to_do::{to_do_factory, ItemTypes};

pub async fn get() -> impl Responder {
    let state: Map<String, Value> = read_file("./state.json");
    let mut array_buffer: Vec<ItemTypes> = Vec::with_capacity(state.len());

    for (key, value) in state {
        let status = TaskStatus::from_string(value.as_str().unwrap().to_string());
        let item = to_do_factory(&key, status);
        array_buffer.push(item);
    }
    web::Json(ToDoItems::new(array_buffer))
}
