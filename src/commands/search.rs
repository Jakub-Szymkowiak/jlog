use crate::db::search;

pub fn run(query: String, tag: Option<String>) {
    search::search_logs(&query, tag.as_deref());
}
