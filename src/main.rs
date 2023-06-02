use clap::Parser;
use std::fs;
use time_management::cli::{Cli, Commands};
use time_management::commands::init::Init;
use time_management::commands::track;
use time_management::config::Config;
use time_management::time_log::{Entry, TimeLog};

fn main() {
    let mut config = Config::init();
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
                let (project, task) = Entry::parse_project_task(project_task.to_owned());
                let mut parsed_amount = 0;
                if let Some(string_amount) = amount {
                    parsed_amount = Entry::to_raw_time(string_amount.to_owned());
                }

                let entry = Entry::new(
                    project,
                    task,
                    parsed_amount,
                    chrono::Local::now(),
                    None,
                    message.clone(),
                );
                let track = track::Track::new(&config, entry).run();
            }
            Commands::Report {
                project,
                task,
                start,
                end,
            } => {
                let (path, file) = config.today_path().unwrap();
                let entries = TimeLog::from(path.join(file)).unwrap();
                let total = entries.total_time();
                println!("Total: {}", Entry::to_string_time(total));
                println!("{:?}", entries);
                println!("Report");
            }
        },
        None => {}
    }
}
