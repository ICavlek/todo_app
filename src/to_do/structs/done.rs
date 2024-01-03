use super::super::enums::TaskStatus;
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

#[test]
fn test_task_done_values() {
    let done = Done::new("Done");
    assert_eq!(
        done.super_struct.status.stringify(),
        TaskStatus::Done.to_string()
    );
}
