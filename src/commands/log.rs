use crate::db::logs;

pub fn run(args: Vec<String>) {
    if args.len() < 2 {
        eprintln!("Usage: jlog <category> <message> [-t tag]");
        return;
    }

    let category = &args[0];
    let mut message_parts = Vec::new();
    let mut tags = Vec::new();

    let mut iter = args[1..].iter().peekable();
    while let Some(arg) = iter.next() {
        if arg == "-t" || arg == "--tag" {
            if let Some(tag) = iter.next() {
                tags.push(tag.clone());
            }
        } else {
            message_parts.push(arg.clone());
        }
    }

    let message = message_parts.join(" ");
    logs::add_log(category, &message, &tags);
}
