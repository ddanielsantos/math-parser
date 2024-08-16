use clap::{Parser, Subcommand};
use math_parser::tokenize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    Tokenize {
        /// Math expression to be parsed
        input: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Tokenize { input }) => println!("{:?}", tokenize(input.as_str())),
        None => todo!(),
    }
}
