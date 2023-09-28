use std::path::PathBuf;
use clap::{arg, Command};
mod prompt;

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
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let name = sub_matches.get_one::<String>("NAME").expect("required");
            let prompt = sub_matches.get_one::<String>("PROMPT").expect("required");
            let num_params = prompt.matches(|c| c == '[').count();
            println!("Added prompt \"{}\" with template \"{}\" with {num_params} parameters", name, prompt);
        }
        _ => unreachable!(),
    }
}
