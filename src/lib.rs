use std::collections::HashMap;

type JsonNumber = u64;

pub enum JsonValue {
    JsonBool(bool),
    JsonString(String),
    // Support only unsigned ints for now
    JsonNumber(JsonNumber),
    JsonNull,
    JsonArray(Vec<JsonValue>),
    JsonObject(HashMap<String, JsonValue>),
}

#[derive(Debug, PartialEq)]
pub enum JsonToken<'a> {
    RightBracket,
    LeftBracket,
    Comma,
    String(&'a [u8]),
    Number(&'a [u8]),
    JsonNull,
    JsonBool(&'a [u8]),
}

fn parse_char(input: &[u8], c: u8) -> Option<(&[u8], u8)> {
    if *input.first()? == c {
        Some((&input[1..], c))
    } else {
        None
    }
}

fn parse_string<'a>(input: &'a [u8], s: &'a [u8]) -> Option<(&'a [u8], &'a [u8])> {
    if input.starts_with(s) {
        Some((&input[s.len()..], s))
    } else {
        None
    }
}

fn parse_null(input: &[u8]) -> Option<JsonToken> {
    parse_string(input, b"null").map(|_| JsonToken::JsonNull)
}

fn parse_bool(input: &[u8]) -> Option<JsonToken> {
    parse_string(input, "true".as_bytes())
        .or(parse_string(input, b"false"))
        .map(|(_, b)| JsonToken::JsonBool(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_char() {
        let input = "hi".as_bytes();
        assert_eq!(parse_char(input, b'h'), Some(("i".as_bytes(), b'h')));
        let input = "i".as_bytes();
        assert_eq!(parse_char(input, b'i'), Some(("".as_bytes(), b'i')));
        let input = "i".as_bytes();
        assert_eq!(parse_char(input, b't'), None);
        let input = "".as_bytes();
        assert_eq!(parse_char(input, b't'), None);
    }

    #[test]
    fn test_parse_string() {
        let input = b"nullable";
        assert_eq!(
            parse_string(input, b"null"),
            Some(("able".as_bytes(), "null".as_bytes()))
        );
        let input = "nulable".as_bytes();
        assert_eq!(parse_string(input, b"null"), None);
        let input = "".as_bytes();
        assert_eq!(parse_string(input, b"null"), None);
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(parse_bool(b"true"), Some(JsonToken::JsonBool(b"true")));
        assert_eq!(parse_bool(b"false"), Some(JsonToken::JsonBool(b"false")));
        assert_eq!(parse_bool(b"null"), None);
    }
}
