use crate::db::{connect, state};
use rusqlite::params;

pub fn set_current_project(name: &str) {
    let conn = connect();

    conn.execute(
        "INSERT OR IGNORE INTO projects (name) VALUES (?1)",
        params![name],
    )
    .expect("failed to insert project");

    state::set("current_project", name);
}

pub fn delete_project(name: &str) {
    let conn = connect();

    conn.execute(
        "DELETE FROM logs WHERE project_id = (SELECT id FROM projects WHERE name = ?1)",
        params![name],
    )
    .expect("failed to delete logs");

    conn.execute("DELETE FROM projects WHERE name = ?1", params![name])
        .expect("failed to delete project");

    // If the current project was deleted, clear the state
    state::clear_if("current_project", name);
}

pub fn list_projects() -> Vec<String> {
    let conn = connect();

    let mut stmt = conn
        .prepare("SELECT name FROM projects ORDER BY name")
        .expect("failed to prepare statement");

    let rows = stmt
        .query_map([], |row| row.get(0))
        .expect("failed to query projects");

    rows.filter_map(Result::ok).collect()
}
