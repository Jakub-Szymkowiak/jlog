use crate::db::logs;

pub fn run(category: Option<String>) {
    match category {
        Some(cat) => logs::show_logs(&cat),
        None => logs::show_all_logs(),
    }
}
