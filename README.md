### qlg

`qlg` (*quick log*) is a simple terminal-based tool for structured project logging.  
Think todos, notes, debug info — all tagged, timestamped, and organized per project.

Built in Rust, zero config, local SQLite storage.

#### Installation
`qlg` is available via `cargo`:
```bash
cargo install qlg
```

#### Usage
###### Quick Start
```
qlg project set my-qlg-project # Set up a new (or switch to) a project
qlg todo "a new todo"          # Log under 'todo' category
qlg show                       # View logs for current project
```
