use crate::db::projects;

pub fn run() {
    let projects = projects::list_projects();

    if projects.is_empty() {
        println!("No projects found.");
    } else {
        println!("Projects:");
        for name in projects {
            println!("- {}", name);
        }
    }
}
