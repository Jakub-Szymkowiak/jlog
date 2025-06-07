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

fn db_path() -> PathBuf {
    let mut path = data_dir().expect("could not get data dir");
    path.push("jlog");
    fs::create_dir_all(&path).expect("could not create data dir");
    path.push("jlog.db");
    path
}

pub fn connect() -> Connection {
    Connection::open(db_path()).expect("failed to open db")
}
