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
