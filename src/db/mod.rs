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
    let dir = base.join("jlog-cli");
    fs::create_dir_all(&dir).expect("Failed to create jlog-cli data directory");
    dir.join("jlog.db")
}

pub fn connect() -> Connection {
    Connection::open(db_path()).expect("Failed to open database")
}
