use crate::db::init;

pub fn run() {
    init::init_db();
    println!("Initialized jlog database.");
}
