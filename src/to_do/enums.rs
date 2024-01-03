use std::fmt;

pub enum TaskStatus {
    Done,
    Pending,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Done => {
                write!(f, "DONE")
            }
            Self::Pending => {
                write!(f, "PENDING")
            }
        }
    }
}

#[test]
fn test_task_status_values() {
    assert_eq!(TaskStatus::Done.to_string(), "DONE");
    assert_eq!(TaskStatus::Pending.to_string(), "PENDING");
}
