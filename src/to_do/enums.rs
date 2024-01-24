use std::fmt;

pub enum TaskStatus {
    Done,
    Pending,
}

impl TaskStatus {
    pub fn stringify(&self) -> String {
        match self {
            Self::Done => "DONE".to_string(),
            Self::Pending => "PENDING".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "DONE" => TaskStatus::Done,
            "PENDING" => TaskStatus::Pending,
            _ => panic!("input {} not supported", input_string),
        }
    }
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

#[cfg(test)]
mod tests {
    use crate::to_do::enums::TaskStatus;

    #[test]
    fn test_task_status_values() {
        assert_eq!(TaskStatus::Done.to_string(), "DONE");
        assert_eq!(TaskStatus::Pending.to_string(), "PENDING");
        assert_eq!(TaskStatus::Done.stringify(), "DONE");
        assert_eq!(TaskStatus::Pending.stringify(), "PENDING");
    }
}
