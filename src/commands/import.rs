use crate::db::logs;
use std::path::Path;

pub fn run(path: String) {
    let path = Path::new(&path);
    let ext = path.extension().and_then(|e| e.to_str());

    match ext {
        Some("json") => logs::import_json(path),
        _ => eprintln!("Unsupported import file (expected .json)"),
    }
}
