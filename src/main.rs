use json_parser_rs::parse_json_value;

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let (_, json) = parse_json_value(input.as_bytes()).unwrap();
    println!("{:?}", json)
}
