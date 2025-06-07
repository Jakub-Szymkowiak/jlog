mod commands;
mod db;

use clap::{Parser, Subcommand};
use commands::project::ProjectCommands;
use commands::{export, import, init, list, log, project, remove, search, show};

#[derive(Parser)]
#[command(
    name = "qlg",
    about = "Log quick notes. Usage: qlg <category> <message>."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Initialize the jlog database")]
    Init,

    #[command(about = "Log a message under a category. Usage: jlog <category> <message> [#tags...]")]
    #[command(external_subcommand)]
    Log(Vec<String>),

    #[command(about = "List all categories in the current project")]
    List,

    #[command(about = "Manage projects (set/delete)")]
    Project {
        #[command(subcommand)]
        subcommand: ProjectCommands,
    },

    #[command(about = "Show logs by category, or all logs if no category is provided")]
    Show {
        #[arg(required = false, help = "Optional category to filter logs")]
        category: Option<String>,
    },

    #[command(about = "Export logs in JSON or Markdown format")]
    Export {
        #[arg(help = "Export format: json or md")]
        format: String,

        #[arg(help = "Target directory path")]
        path: String,
    },

    #[command(about = "Import logs from a JSON file")]
    Import {
        #[arg(help = "Path to the JSON file")]
        path: String,
    },

    #[command(about = "Remove a log entry by ID")]
    Remove {
        #[arg(help = "ID of the log to remove")]
        id: i64,
    },

    #[command(about = "Search logs by query and optional tag")]
    Search {
        #[arg(required = false, help = "Message content to search for")]
        query: String,

        #[arg(short, long, help = "Tag to filter by")]
        tag: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init::run(),
        Commands::Log(args) => log::run(args),
        Commands::List => list::run(),
        Commands::Project { subcommand } => project::run(subcommand),
        Commands::Show { category } => show::run(category),
        Commands::Export { format, path } => export::run(format, path),
        Commands::Import { path } => import::run(path),
        Commands::Remove { id } => remove::run(id),
        Commands::Search { query, tag } => search::run(query, tag),
    }
}
