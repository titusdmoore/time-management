use clap::Parser;
use time_management::cli::{Cli, Commands};
use time_management::commands::init::Init;
use time_management::commands::track;

fn main() {
    let cli = Cli::parse();

    if let Some(test) = cli.test.as_deref() {
        println!("Test: {}", test);
    }

    match &cli.commands {
        Some(commands) => match commands {
            Commands::Init { path } => {
                let init_result = Init::new(path.clone()).run();

                match init_result {
                    Ok(_) => {
                        println!("Time Management has been initialized!");
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            Commands::Track {
                project_task,
                message,
                amount,
            } => {
                let track = track::Track::new(
                    project_task.clone(),
                    message.clone(),
                    amount.clone().unwrap_or("0:0".to_string()),
                )
                .run();
            }
        },
        None => {}
    }
}
