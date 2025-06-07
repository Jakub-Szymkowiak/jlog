use crate::db::projects;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ProjectCommands {
    Set { name: String },
    Delete { name: String },
}

pub fn run(subcommand: ProjectCommands) {
    match subcommand {
        ProjectCommands::Set { name } => {
            projects::set_current_project(&name);
            println!("Current project set to '{}'", name);
        }
        ProjectCommands::Delete { name } => {
            projects::delete_project(&name);
            println!("Deleted project '{}'", name);
        }
    }
}
