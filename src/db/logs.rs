use crate::db::{connect, utils};
use chrono::{DateTime, Utc};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub fn add_log(category: &str, message: &str, tags: &[String]) {
    let conn = connect();
    let project_id = utils::get_current_project_id(&conn).expect("no active project set");

    let timestamp = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO logs (category, message, timestamp, project_id)
         VALUES (?1, ?2, ?3, ?4)",
        params![category, message, timestamp, project_id],
    )
    .expect("failed to insert log");

    let log_id = conn.last_insert_rowid();

    for tag in tags {
        conn.execute(
            "INSERT INTO tags (log_id, name) VALUES (?1, ?2)",
            params![log_id, tag],
        )
        .expect("failed to insert tag");
    }
}

pub fn show_logs(category: &str) {
    let conn = connect();
    let project_id = utils::get_current_project_id(&conn).expect("no active project set");

    let mut stmt = conn
        .prepare(
            "SELECT id, timestamp, category, message FROM logs
         WHERE project_id = ?1 AND category = ?2
         ORDER BY timestamp DESC",
        )
        .expect("failed to prepare query");

    let logs = stmt
        .query_map(params![project_id, category], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .expect("failed to query logs");

    for log in logs {
        let (id, ts, cat, msg) = log.expect("row error");
        let tags = utils::get_tags(&conn, id);
        println!("{}", utils::format_log_entry(id, &ts, &cat, &msg, &tags));
    }
}

pub fn show_all_logs() {
    let conn = connect();
    let project_id = utils::get_current_project_id(&conn).expect("no active project set");

    let mut stmt = conn
        .prepare(
            "SELECT id, timestamp, category, message FROM logs
         WHERE project_id = ?1
         ORDER BY timestamp DESC",
        )
        .expect("failed to prepare query");

    let logs = stmt
        .query_map(params![project_id], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .expect("failed to query logs");

    for log in logs {
        let (id, ts, cat, msg) = log.expect("row error");
        let tags = utils::get_tags(&conn, id);
        println!("{}", utils::format_log_entry(id, &ts, &cat, &msg, &tags));
    }
}

pub fn delete_log(id: i64) {
    let conn = connect();

    conn.execute("DELETE FROM logs WHERE id = ?1", params![id])
        .expect("failed to delete log");

    conn.execute("DELETE FROM tags WHERE log_id = ?1", params![id])
        .expect("failed to delete tags");
}

#[derive(Serialize)]
struct LogEntry {
    category: String,
    message: String,
    timestamp: String,
    tags: Vec<String>,
}

pub fn export_json(path: &Path) {
    let conn = connect();
    let project_id = utils::get_current_project_id(&conn).expect("no active project");

    let mut stmt = conn
        .prepare("SELECT id, category, message, timestamp FROM logs WHERE project_id = ?1 ORDER BY timestamp ASC")
        .expect("failed to prepare");

    let logs: Vec<LogEntry> = stmt
        .query_map([project_id], |row| {
            let id: i64 = row.get(0)?;
            let tags = utils::get_tags(&conn, id);
            Ok(LogEntry {
                category: row.get(1)?,
                message: row.get(2)?,
                timestamp: row.get(3)?,
                tags,
            })
        })
        .expect("failed to fetch")
        .map(|r| r.expect("row error"))
        .collect();

    let json = serde_json::to_string_pretty(&logs).expect("serialization failed");
    fs::write(path, json).expect("write failed");
    println!("Exported logs to {}", path.display());
}

pub fn export_markdown(path: &Path) {
    let conn = connect();
    let project_id = utils::get_current_project_id(&conn).expect("no active project");

    let mut stmt = conn
        .prepare("SELECT id, category, message, timestamp FROM logs WHERE project_id = ?1 ORDER BY timestamp ASC")
        .expect("prepare failed");

    let logs = stmt
        .query_map([project_id], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .expect("query failed");

    let mut output = String::new();
    for log in logs {
        let (id, cat, msg, ts_raw) = log.expect("row error");
        let tags = utils::get_tags(&conn, id);
        let tags_str = tags
            .iter()
            .map(|t| format!("#{}", t))
            .collect::<Vec<_>>()
            .join(" ");

        let ts = DateTime::parse_from_rfc3339(&ts_raw)
            .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or(ts_raw);

        output.push_str(&format!("### [{}] {} {}\n{}\n\n", ts, cat, tags_str, msg));
    }

    fs::write(path, output).expect("write failed");
    println!("Exported logs to {}", path.display());
}

#[derive(Deserialize)]
struct ImportedLog {
    category: String,
    message: String,
    timestamp: String,
    #[serde(default)]
    tags: Vec<String>,
}

pub fn import_json(path: &Path) {
    let data = fs::read_to_string(path).expect("failed to read file");
    let logs: Vec<ImportedLog> = serde_json::from_str(&data).expect("invalid JSON format");

    let conn = connect();
    let project_id = utils::get_current_project_id(&conn).expect("no active project");

    for log in &logs {
        if DateTime::parse_from_rfc3339(&log.timestamp).is_err() {
            panic!("invalid timestamp format in import: {}", log.timestamp);
        }

        conn.execute(
            "INSERT INTO logs (category, message, timestamp, project_id)
             VALUES (?1, ?2, ?3, ?4)",
            params![log.category, log.message, log.timestamp, project_id],
        )
        .expect("failed to insert log");

        let log_id = conn.last_insert_rowid();

        for tag in &log.tags {
            conn.execute(
                "INSERT INTO tags (log_id, name) VALUES (?1, ?2)",
                params![log_id, tag],
            )
            .expect("failed to insert tag");
        }
    }

    println!("Imported {} logs from JSON", logs.len());
}
