use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ParserType {
    Normal,
    Nom,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    parser: ParserType,
}

fn main() {
    let args = Args::parse();
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let json = match args.parser {
        ParserType::Normal => format!("{:?}", json_parser_rs::parse_json(&input)),
        ParserType::Nom => format!("{:?}", json_parser_rs::nom_json::parse_json(&input)),
    };
    println!("{:?}", json)
}
