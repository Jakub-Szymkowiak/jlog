use crate::db::state;

use chrono::NaiveDateTime;
use rusqlite::{Connection, params};

pub fn get_current_project_id(conn: &Connection) -> Option<i64> {
    let name = state::get("current_project")?;

    conn.query_row(
        "SELECT id FROM projects WHERE name = ?1",
        params![name],
        |row| row.get(0),
    )
    .ok()
}

pub fn get_tags(conn: &Connection, log_id: i64) -> Vec<String> {
    let mut stmt = conn
        .prepare("SELECT name FROM tags WHERE log_id = ?1")
        .unwrap();
    stmt.query_map(params![log_id], |row| row.get(0))
        .unwrap()
        .map(Result::unwrap)
        .collect()
}

pub fn format_log_entry(
    id: i64,
    timestamp: &str,
    category: &str,
    message: &str,
    tags: &[String]) -> String {
    let ts = NaiveDateTime::parse_from_str(timestamp, "%+")
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|_| timestamp.to_string());

    let tag_str = if tags.is_empty() {
        String::new()
    } else {
        format!(" {}", tags.iter().map(|t| format!("#{}", t)).collect::<Vec<_>>().join(" "))
    };

    format!("[{}] {} {}: {}{}", id, ts, category, message, tag_str)
}