pub mod init;
pub mod logs;
pub mod projects;
pub mod search;
pub mod state;
pub mod utils;

use dirs::data_dir;
use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;


pub fn db_path() -> PathBuf {
    let base = data_dir().expect("Could not resolve user data directory");
    let dir = base.join("qlg");
    fs::create_dir_all(&dir).expect("Failed to create qlg-cli data directory");
    dir.join("qlg.db")
}

pub fn connect() -> Connection {
    let path = db_path();
    let first_time = !path.exists();

    fs::create_dir_all(path.parent().unwrap()).expect("failed to create db directory");
    let conn = Connection::open(path).expect("failed to open db");

    if first_time {
        crate::db::init::init_db();
    }

    conn
}
