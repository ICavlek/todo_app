use super::super::enums::TaskStatus;
use super::super::traits::delete::Delete;
use super::super::traits::edit::Edit;
use super::super::traits::get::Get;
use super::base::Base;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::Done,
        };
        Done { super_struct: base }
    }
}

impl Get for Done {}
impl Delete for Done {}
impl Edit for Done {}

#[test]
fn test_task_done_values() {
    let done = Done::new("Done");
    assert_eq!(
        done.super_struct.status.stringify(),
        TaskStatus::Done.to_string()
    );
}
