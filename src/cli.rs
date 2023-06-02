use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author = "Titus Moore", version = "0.1.0", about = "A simple time management cli tool", long_about = None)]
pub struct Cli {
    /// Test Parameter
    #[arg(short = 't', long = "test", help = "Test parameter")]
    pub test: Option<String>,

    #[command(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        #[arg(
            short = 'p',
            long = "path",
            value_name = "<PATH>",
            help = "Optional: Alternative path to initialize the config files at."
        )]
        path: Option<PathBuf>,
    },
    Track {
        project_task: String,
        #[arg(
            short = 'm',
            long = "message",
            value_name = "<MESSAGE>",
            help = "Optional: Message to be associated with the tracked time."
        )]
        message: Option<String>,
        // This is a string arg, TODO: implement a way to parse this into a time object
        #[arg(
            short = 'a',
            long = "amount",
            value_name = "<AMOUNT>",
            help = "Optional: Amount of time to be tracked."
        )]
        amount: Option<String>,
    },
    Report {
        #[arg(
            short = 'p',
            long = "project",
            value_name = "<PROJECT>",
            help = "Optional: Project to report on."
        )]
        project: Option<String>,
        #[arg(
            short = 't',
            long = "task",
            value_name = "<TASK>",
            help = "Optional: Task to report on."
        )]
        task: Option<String>,
        #[arg(
            short = 's',
            long = "start",
            value_name = "<START>",
            help = "Optional: Start date to report on."
        )]
        start: Option<String>,
        #[arg(
            short = 'e',
            long = "end",
            value_name = "<END>",
            help = "Optional: End date to report on."
        )]
        end: Option<String>,
    },
}
