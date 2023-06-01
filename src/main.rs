use clap::Parser;
use std::fs;
use time_management::cli::{Cli, Commands};
use time_management::commands::init::Init;
use time_management::commands::track;
use time_management::config::Config;
use time_management::entries::Entries;

fn main() {
    let mut config = Config::init();
    if let Some(mut work_path) = config.work_path.clone() {
        work_path.push("time-management/test.toml");
        let entries_string = fs::read_to_string(work_path).unwrap();
        let entries: Entries = toml::from_str(&entries_string).unwrap();
        println!("{:?}", entries);
    }
    let cli = Cli::parse();

    if let Some(test) = cli.test.as_deref() {
        println!("Test: {}", test);
    }

    match &cli.commands {
        Some(commands) => match commands {
            Commands::Init { path } => {
                let init_result = Init::new(path.clone(), &mut config).run();

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
                .run(&config);
            }
        },
        None => {}
    }
}
