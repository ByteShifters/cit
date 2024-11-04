use clap::{Command, Arg};
use log::error;
use std::process::exit;

mod commands;

fn main() {
    env_logger::init();

    let matches = Command::new("CIT")
        .version("1.0")
        .author("ren <ren@byteshifters.com>")
        .about("A Rust-based Git wrapper with logging")
        .subcommand(Command::new("commit")
            .about("Commits with a meaningful commit message")
            .arg(Arg::new("message").short('m').long("message").required(true).help("Commit message")))
        .subcommand(Command::new("push").about("Pushes the current branch"))
        .subcommand(Command::new("upd").about("Adds all changes"))
        .subcommand(Command::new("undo").about("Undoes the last commit"))
        .subcommand(Command::new("log").about("Shows the git log"))
        .subcommand(Command::new("switch")
            .about("Switches branches, creating if needed")
            .arg(Arg::new("branch").short('b').long("branch").required(true).help("Branch name")))
        .subcommand(Command::new("diff").about("Shows the git diff"))
        .subcommand(Command::new("upload")
            .about("Add, commit, and push changes")
            .arg(Arg::new("message").short('m').long("message").required(true).help("Commit message"))
            .arg(Arg::new("branch").short('b').long("branch").required(true).help("Branch to push")))
        .get_matches();

    if matches.subcommand_name().is_none() {
        print_custom_help();
        exit(0);
    }

    match matches.subcommand() {
        Some(("commit", sub_m)) => {
            let msg = sub_m.get_one::<String>("message").expect("Expected a message argument");
            execute_command(|| commands::commit::commit::commit_all(msg))
                .unwrap_or_else(|_| exit(1));
        },
        Some(("push", _)) => {
            execute_command(commands::push::push::push_all)
                .unwrap_or_else(|_| exit(1));
        },
        Some(("upd", _)) => {
            execute_command(commands::add::add::add_all)
                .unwrap_or_else(|_| exit(1));
        },
        Some(("undo", _)) => {
            execute_command(commands::undo::undo::undo_last)
                .unwrap_or_else(|_| exit(1));
        },
        Some(("log", _)) => {
            execute_command(commands::log::log::show_log)
                .unwrap_or_else(|_| exit(1));
        },
        Some(("switch", sub_m)) => {
            let branch = sub_m.get_one::<String>("branch").expect("Expected a branch argument");
            execute_command(|| commands::switch::switch::switch_branch(branch))
                .unwrap_or_else(|_| exit(1));
        },
        Some(("diff", _)) => {
            execute_command(commands::diff::diff::show_diff)
                .unwrap_or_else(|_| exit(1));
        },
        Some(("upload", sub_m)) => {
            let msg = sub_m.get_one::<String>("message").expect("Expected a message argument");
            let branch = sub_m.get_one::<String>("branch").expect("Expected a branch argument");
            execute_command(|| commands::upload::upload::upload_changes(msg, branch))
                .unwrap_or_else(|_| exit(1));
        },
        _ => {
            eprintln!("No valid command was used. Please run `cit --help` for usage information.");
            exit(1);
        }
    }
}

fn print_custom_help() {
    println!("Usage: cit [COMMAND]");
    println!();
    println!("Available commands:");
    println!("{:<10}  {}", "commit", "Commits with a meaningful commit message");
    println!("{:<10}  {}", "push", "Pushes the current branch");
    println!("{:<10}  {}", "upd", "Adds all changes");
    println!("{:<10}  {}", "undo", "Undoes the last commit");
    println!("{:<10}  {}", "log", "Shows the git log");
    println!("{:<10}  {}", "switch", "Switches branches, creating if needed");
    println!("            Arguments:");
    println!("            -b, --branch  Branch name (required)");
    println!();
    println!("{:<10}  {}", "diff", "Shows the git diff");
    println!();
    println!("{:<10}  {}", "upload", "Add, commit, and push changes");
    println!("            Arguments:");
    println!("            -m, --message  Commit message (required)");
    println!("            -b, --branch   Branch to push (required)");
    println!();
}

fn execute_command<F>(cmd: F) -> Result<(), String>
where
    F: FnOnce() -> Result<(), String>,
{
    cmd().map_err(|e| {
        error!("(ERR): {} - Failed: {}", chrono::Utc::now(), e);
        e
    })?;
    Ok(())
}
