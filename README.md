# JSON parser and formatter written in rust for educational purposes

There are two implementations of the JSON parser. The first one is the hand-rolled version I wrote myself while the second is written using the [nom-rs](https://github.com/rust-bakery/nom) library.

This project is for my understanding of how to implement basic tokenization and parsing, as well as how to use a parser combinator library like nom.

## Data Structures

```rust
pub enum Number<'a> {
    Float(&'a str),
    Int(&'a str),
}

pub enum JsonValue<'a> {
    Bool(bool),
    String(&'a str),
    Number(Number<'a>),
    Null,
    Array(Vec<JsonValue<'a>>),
    Object(Vec<(&'a str, JsonValue<'a>)>),
}
```

The data structure for a JSON document is quite straightforward.
A `Vec` of pairs is used to represent JSON objects instead of a `HashMap` in order to preserve the key-value order of the object.
A JSON number can be in multiple formats such as `123`, `1.23`, `1e8` etc. so that is also taken into account in the nom parser.
The hand-rolled parser, however, only supports integer numbers.

## Library Usage

```rust
use json_parser_rs;

fn main() {
    let input = r#"{"foo": "bar"}"#;
    let parsed_json_default = parse_json(&input); // hand-rolled parser
    let parsed_json_nom = nom_json::parse_json(&input); // parser written with nom
    println!("{:?}", parsed_json_default); // Object([("foo", String("bar"))])
    println!("{:?}", parsed_json_nom); // Object([("foo", String("bar"))])
}
```

## Formatter CLI Usage

```bash
# using stdin
cat '{"foo":"bar"}' | json_parser_rs
# passing in a file
json_parser_rs foo.json
# compact mode
json_parser_rs -c foo.json
# custom indent spaces (number of spaces each level of indentation has, default is 2 spaces)
json_parser_rs --indent-spaces 4 foo.json
```

## Caveats

The parser/formatter is very basic and definitely does not implement the entire JSON spec.
String escapes has also not been implemented.
