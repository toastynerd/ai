use std::path::PathBuf;
use std::env;
use clap::{arg, Command};
use sled;
mod prompt;
mod data_store;
mod template;

fn cli() -> Command {
    Command::new("prompt")
        .about("Interface with chatGPT prompts")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .subcommand(
            Command::new("add")
                .about("Add a prompt")
                .arg(arg!(<NAME> "the name of the prompt"))
                .arg(arg!(<PROMPT> "the prompt template"))
        )

}

fn main() {
    if env::var("PROMPT_DB_NAME").is_err() {
        env::set_var("PROMPT_DB_NAME", "db/prompt_db");
    }

    let db = sled::open(env::var("PROMPT_DB_NAME").unwrap_or_default()).unwrap();
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let new_prompt = prompt::create_prompt(
                sub_matches.get_one::<String>("NAME").expect("required").to_string(),
                sub_matches.get_one::<String>("PROMPT").expect("required").to_string(),
            );
            data_store::store_prompt(&db, &new_prompt);
            println!("Added prompt \"{}\" with template \"{}\" with {} parameters", new_prompt.name, new_prompt.template, new_prompt.number_of_parameters);
        }
        _ => unreachable!(),
    }
}
