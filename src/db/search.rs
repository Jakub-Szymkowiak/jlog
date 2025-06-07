use crate::db::{connect, utils};
use rusqlite::params;

pub fn search_logs(query: &str, tag: Option<&str>) {
    let conn = connect();
    let project_id = utils::get_current_project_id(&conn).expect("no active project set");

    let sql = match tag {
        Some(_) => {
            "SELECT id, timestamp, category, message FROM logs
             WHERE project_id = ?1 AND message LIKE ?2 AND EXISTS (
                 SELECT 1 FROM tags WHERE tags.log_id = logs.id AND tags.name = ?3
             )
             ORDER BY timestamp DESC"
        }
        None => {
            "SELECT id, timestamp, category, message FROM logs
             WHERE project_id = ?1 AND message LIKE ?2
             ORDER BY timestamp DESC"
        }
    };

    let mut stmt = conn.prepare(sql).expect("failed to prepare statement");

    let pattern = format!("%{}%", query);

    let rows = match tag {
        Some(tag) => stmt.query_map(params![project_id, pattern, tag], map_row),
        None => stmt.query_map(params![project_id, pattern], map_row),
    }
    .expect("query failed");

    for row in rows {
        let (id, ts, cat, msg) = row.expect("row error");
        let tags = utils::get_tags(&conn, id);
        println!("{}", utils::format_log_entry(id, &ts, &cat, &msg, &tags));
    }
}

fn map_row(row: &rusqlite::Row) -> rusqlite::Result<(i64, String, String, String)> {
    let id: i64 = row.get(0)?;
    let ts: String = row.get(1)?;
    let cat: String = row.get(2)?;
    let msg: String = row.get(3)?;
    Ok((id, ts, cat, msg))
}
