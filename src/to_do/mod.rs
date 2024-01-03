pub mod enums;
pub mod structs;
pub mod traits;

use enums::TaskStatus;
use structs::done::Done;
use structs::pending::Pending;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}
pub fn to_do_factory(title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::Done => ItemTypes::Done(Done::new(title)),
        TaskStatus::Pending => ItemTypes::Pending(Pending::new(title)),
    }
}

#[test]
fn test_to_do_factory() {
    let pending_task = to_do_factory("Washing", TaskStatus::Pending);
    let done_task = to_do_factory("Shopping", TaskStatus::Done);
    match pending_task {
        ItemTypes::Pending(task) => assert_eq!(
            task.super_struct.status.stringify(),
            task.super_struct.status.to_string()
        ),
        _ => {}
    };

    match done_task {
        ItemTypes::Done(task) => assert_eq!(
            task.super_struct.status.stringify(),
            task.super_struct.status.to_string()
        ),
        _ => {}
    };
}
