mod state;
mod to_do;

use std::env;

use serde_json::{Map, Value, json};
use state::read_file;
use to_do::{
    enums::TaskStatus,
    to_do_factory,
    traits::{delete::Delete, edit::Edit, get::Get},
    ItemTypes,
};

use crate::state::write_to_file;

fn to_do_factory_func() {
    let to_do_item = to_do_factory("Washing", TaskStatus::Done);
    match to_do_item {
        ItemTypes::Done(item) => {
            println!(
                "{} is {}",
                item.super_struct.title,
                item.super_struct.status.stringify()
            );
            item.get(&item.super_struct.title);
            item.delete(&item.super_struct.title);
        }
        ItemTypes::Pending(item) => {
            item.get(&item.super_struct.title);
            item.set_to_done(&item.super_struct.title);
        }
    }
}

fn reading_writing_json() {
    let args: Vec<String> = env::args().collect();
    let status: &String = &args[1];
    let title: &String = &args[2];
    let mut state: Map<String, Value> =
    read_file("./state.json");
    println!("Before operation: {:?}", state);
    state.insert(title.to_string(), json!(status));
    println!("After operation: {:?}", state);
    write_to_file("./state.json", &mut state);
}

fn main() {
    to_do_factory_func();
    println!("------------");
    reading_writing_json();
}
