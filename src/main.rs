mod to_do;
use to_do::enums::TaskStatus;

fn main() {
    println!("{}", TaskStatus::Done);
    println!("{}", TaskStatus::Pending);
    let outcome = TaskStatus::Done.to_string();
    println!("{}", outcome);
}
