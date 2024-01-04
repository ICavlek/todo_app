mod to_do;

use to_do::{
    enums::TaskStatus,
    to_do_factory,
    traits::{delete::Delete, edit::Edit, get::Get},
    ItemTypes,
};

fn main() {
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
