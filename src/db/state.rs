use crate::db::connect;
use rusqlite::params;

pub fn set(key: &str, value: &str) {
    let conn = connect();
    conn.execute(
        "INSERT INTO state (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
    )
    .expect("failed to set state");
}

pub fn get(key: &str) -> Option<String> {
    let conn = connect();
    conn.query_row(
        "SELECT value FROM state WHERE key = ?1",
        params![key],
        |row| row.get(0),
    )
    .ok()
}

pub fn clear_if(key: &str, expected_value: &str) {
    let conn = connect();
    conn.execute(
        "DELETE FROM state WHERE key = ?1 AND value = ?2",
        params![key, expected_value],
    )
    .ok();
}
