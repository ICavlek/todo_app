use serde::Serialize;

use crate::to_do::{structs::base::Base, ItemTypes};

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_array_buffer: Vec<Base> = Vec::with_capacity(input_items.len());
        let mut done_array_buffer: Vec<Base> = Vec::with_capacity(input_items.len());

        for item in input_items {
            match item {
                ItemTypes::Pending(packed) => pending_array_buffer.push(packed.super_struct),
                ItemTypes::Done(packed) => done_array_buffer.push(packed.super_struct),
            }
        }

        let pending_count = pending_array_buffer.len() as i8;
        let done_count = done_array_buffer.len() as i8;

        ToDoItems {
            pending_items: pending_array_buffer,
            done_items: done_array_buffer,
            pending_item_count: pending_count,
            done_item_count: done_count,
        }
    }
}
