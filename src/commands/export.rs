use crate::db::logs;
use std::path::PathBuf;

pub fn run(format: String, dir: String) {
    let path = PathBuf::from(&dir);
    if !path.is_dir() {
        eprintln!("Export path must be a directory");
        return;
    }

    let filename = match format.as_str() {
        "md" | "markdown" => "jlog_export.md",
        "json" => "jlog_export.json",
        _ => {
            eprintln!("Unsupported format: '{}'", format);
            return;
        }
    };

    let full_path = path.join(filename);

    match format.as_str() {
        "md" | "markdown" => logs::export_markdown(&full_path),
        "json" => logs::export_json(&full_path),
        _ => unreachable!(),
    }
}
