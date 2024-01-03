use to_do::{enums::TaskStatus, to_do_factory, ItemTypes};

mod to_do;

fn main() {
    let to_do_item = to_do_factory("washing", TaskStatus::Done);
    match to_do_item {
        ItemTypes::Done(item) => {
            println!("{}", item.super_struct.status.stringify());
            println!("{}", item.super_struct.title);
        }
        ItemTypes::Pending(item) => {
            println!("{}", item.super_struct.status.stringify());
            println!("{}", item.super_struct.title);
        }
    }
}
