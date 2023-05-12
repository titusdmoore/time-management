use clap::Parser;
use time_management::cli::{Cli, Commands};
use time_management::commands::init::Init;
fn main() {
    let cli = Cli::parse();

    if let Some(test) = cli.test.as_deref() {
        println!("Test: {}", test);
    }

    match &cli.commands {
        Some(commands) => match commands {
            Commands::Init { path } => {
                let _ = Init::new(path.clone()).run();
            }
        },
        None => {}
    }
}
