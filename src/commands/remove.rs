use crate::db::logs;

pub fn run(id: i64) {
    logs::delete_log(id);
    println!("Deleted log with ID {}", id);
}
