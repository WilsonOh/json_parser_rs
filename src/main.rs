use clap::Parser;
use json_parser_rs::{formatter::format_json, nom_json::parse_json};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "2")]
    indent_spaces: usize,
    file: Option<String>,
    #[arg(long, short, default_value = "false")]
    compact: bool,
}

fn main() {
    let args = Args::parse();
    let input = match args.file {
        Some(file_path) => std::fs::read_to_string(file_path),
        None => std::io::read_to_string(std::io::stdin()),
    }
    .unwrap();
    if let Some(json) = parse_json(&input) {
        let formatted = format_json(&json, args.indent_spaces, 1, args.compact);
        println!("{}", formatted);
    } else {
        eprintln!("invalid json")
    }
}
