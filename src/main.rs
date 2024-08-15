use clap::Parser;
use math_parser::tokenize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Math expression to be parsed
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", tokenize(args.input.as_str()));
}
