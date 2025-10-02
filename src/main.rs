mod cli;
mod example_builder;

use crate::cli::{Cli, CliCommand, CliConfig};
use crate::example_builder::{Client, Error, ResponseValue, types};
use clap::Command;
use serde_json;
use std::path::PathBuf;
use std::process::exit;

struct SimpleConfig;

impl CliConfig for SimpleConfig {
    fn success_item<T>(&self, value: &ResponseValue<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        println!("Success: {:?}", value);
    }

    fn success_no_item(&self, _value: &ResponseValue<()>) {
        println!("Success no item");
    }

    fn error<T>(&self, value: &Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        eprintln!("Error: {:?}", value);
    }

    fn list_start<T>(&self)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
    }

    fn list_item<T>(&self, value: &T)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        println!("Item: {:?}", value);
    }

    fn list_end_success<T>(&self)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
    }

    fn list_end_error<T>(&self, value: &Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug,
    {
        eprintln!("List error: {:?}", value);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new("http://localhost:8080");
    let config = SimpleConfig;
    let cli = Cli::new(client, config);
    let mut uno = Cli::<SimpleConfig>::get_command(CliCommand::Uno).name("uno");

    let two = Cli::<SimpleConfig>::get_command(CliCommand::Two).name("two");
    let matches = Command::new("example")
        .about("Top-level CLI")
        .subcommand(uno)
        .subcommand(two)
        .get_matches();
    match matches.subcommand() {
        Some(("uno", m)) => cli.execute(CliCommand::Uno, &m).await?,
        Some(("two", m)) => cli.execute(CliCommand::Two, &m).await?,
        _ => exit(3),
    };
    Ok(())
}
