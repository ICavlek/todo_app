use super::super::enums::TaskStatus;
use super::base::Base;

use super::super::traits::create::Create;
use super::super::traits::edit::Edit;
use super::super::traits::get::Get;

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::Pending,
        };
        Pending { super_struct: base }
    }
}

impl Get for Pending {}
impl Edit for Pending {}
impl Create for Pending {}

#[cfg(test)]
mod tests {
    use crate::to_do::{enums::TaskStatus, structs::pending::Pending};

    #[test]
    fn test_task_pending_values() {
        let pending = Pending::new("Pending");
        assert_eq!(
            pending.super_struct.status.stringify(),
            TaskStatus::Pending.to_string()
        );
    }
}
