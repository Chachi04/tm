// use std::path::PathBuf;

// use clap::{arg, command, value_parser, ArgAction, Command};
use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use tmux_interface::variables::session::session::SESSION_ALL;
use tmux_interface::{AttachSession, KillSession, NewSession, Sessions};

use dialoguer::console::Term;

mod ensure_dependecies;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Attach to existing session
    A { session: Option<String> },
    /// Attach to existing session
    Attach { session: Option<String> },
    /// Kill existing session
    K { session: Option<String> },
    /// Kill existing session
    Kill { session: Option<String> },
    /// Create new session
    N {
        #[arg(short, long)]
        attach: bool,
    },
    /// Create new session
    New {
        #[arg(short, long)]
        attach: bool,
    },
    /// List existing sessions
    Ls,
    /// List existing sessions
    List,
}

fn main() -> std::io::Result<()> {
    ensure_dependecies::ensure_dependencies();

    let cli = Cli::parse();

    match &cli.command {
        Some(Command::A { session }) | Some(Command::Attach { session }) => {
            if let Some(session) = session {
                AttachSession::new()
                    .target_session(session)
                    .output()
                    .unwrap();
            } else {
                // using tmux_interface get all sessions in a vector of
                let sessions = Sessions::get(SESSION_ALL).unwrap();
                let mut session_names = Vec::new();
                for session in sessions {
                    match session.name {
                        Some(name) => session_names.push(name),
                        None => (),
                    }
                }

                if session_names.is_empty() {
                    println!("No sessions found");
                    return Ok(());
                }

                let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select a session")
                    .default(0)
                    .items(&session_names)
                    .interact_on_opt(&Term::stderr())?;

                match selection {
                    Some(selection) => {
                        AttachSession::new()
                            .target_session(&session_names[selection])
                            .output()
                            .unwrap();
                    }
                    None => println!("No session selected"),
                }
            }
        }
        Some(Command::K { session }) | Some(Command::Kill { session }) => {
            if let Some(session) = session {
                KillSession::new().target_session(session).output().unwrap();
            } else {
                let sessions = Sessions::get(SESSION_ALL).unwrap();
                let mut session_names = Vec::new();
                for session in sessions {
                    match session.name {
                        Some(name) => session_names.push(name),
                        None => (),
                    }
                }

                if session_names.is_empty() {
                    println!("No sessions found");
                    return Ok(());
                }

                let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select a session")
                    .default(0)
                    .items(&session_names)
                    .interact_on_opt(&Term::stderr())?;

                match selection {
                    Some(selection) => {
                        KillSession::new()
                            .target_session(&session_names[selection])
                            .output()
                            .unwrap();
                    }
                    None => println!("No session selected"),
                }
            }
        }
        Some(Command::N { attach }) | Some(Command::New { attach }) => {
            let input = dialoguer::Input::<String>::new()
                .with_prompt("Session name")
                .interact_text()?;
            NewSession::new()
                .detached()
                .session_name(&input)
                .output()
                .unwrap();
            if *attach {
                AttachSession::new()
                    .target_session(&input)
                    .output()
                    .unwrap();
            }
        }
        Some(Command::Ls) | Some(Command::List) => {
            let sessions = Sessions::get(SESSION_ALL).unwrap();
            for session in sessions {
                match session.name {
                    Some(name) => println!("{}", name),
                    None => (),
                }
            }
        }
        None => {
            println!("Creating new session");
        }
    }
    Ok(())
}
