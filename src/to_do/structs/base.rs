use super::super::enums::TaskStatus;

#[allow(dead_code)]
pub struct Base {
    pub title: String,
    pub status: TaskStatus,
}

#[test]
fn test_task_base_values() {
    let task_pending = Base {
        title: "Pending".to_string(),
        status: TaskStatus::Pending,
    };
    let task_done = Base {
        title: "Done".to_string(),
        status: TaskStatus::Done,
    };
    assert_eq!(
        task_pending.status.stringify(),
        TaskStatus::Pending.to_string()
    );
    assert_eq!(task_done.status.stringify(), TaskStatus::Done.to_string());
}
