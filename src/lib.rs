#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Bool(bool),
    String(String),
    // Support only unsigned ints for now
    Number(u64),
    Null,
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
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

fn parse_null(input: &[u8]) -> Option<(&[u8], JsonValue)> {
    parse_string(input, b"null").map(|(rest, _)| (rest, JsonValue::Null))
}

fn parse_bool(input: &[u8]) -> Option<(&[u8], JsonValue)> {
    parse_string(input, b"true")
        .or(parse_string(input, b"false"))
        .map(|(rest, b)| (rest, JsonValue::Bool(b == b"true")))
}

fn parse_string_literal(input: &[u8]) -> Option<(&[u8], String)> {
    let (input, _) = parse_char(input, b'"')?;
    for (index, &ch) in input.iter().enumerate() {
        if let b'"' = ch {
            return Some((
                &input[index + 1..],
                (String::from_utf8((input[..index]).to_vec())).unwrap(),
            ));
        }
    }
    None
}
fn parse_json_string_literal(input: &[u8]) -> Option<(&[u8], JsonValue)> {
    parse_string_literal(input).map(|(rest, s)| (rest, JsonValue::String(s)))
}

pub fn parse_json_value(input: &[u8]) -> Option<(&[u8], JsonValue)> {
    parse_json_string_literal(input)
        .or(parse_null(input))
        .or(parse_json_string_literal(input))
        .or(parse_bool(input))
        .or(parse_array(input))
        .or(parse_number(input))
        .or(parse_object(input))
}

pub fn skip_whitespace(input: &[u8]) -> &[u8] {
    for (i, c) in input.iter().enumerate() {
        if !c.is_ascii_whitespace() {
            return &input[i..];
        }
    }
    &[]
}

pub fn parse_array(input: &[u8]) -> Option<(&[u8], JsonValue)> {
    let (mut input, _) = parse_char(input, b'[')?;
    input = skip_whitespace(input);
    let mut buf = Vec::new();
    while !input.is_empty() {
        if let Some((rest, val)) = parse_json_value(input) {
            buf.push(val);
            input = rest;
        } else {
            return parse_char(input, b']').map(|(rest, _)| (rest, JsonValue::Array(buf)));
        }
        input = skip_whitespace(input);
        if let Some((rest, _)) = parse_char(input, b',') {
            input = rest;
        }
        input = skip_whitespace(input);
    }
    None
}

pub fn parse_object(input: &[u8]) -> Option<(&[u8], JsonValue)> {
    let (mut input, _) = parse_char(input, b'{')?;
    input = skip_whitespace(input);
    let mut m = Vec::new();
    while !input.is_empty() {
        input = skip_whitespace(input);
        if let Some((rest, _)) = parse_char(input, b'}') {
            return Some((rest, JsonValue::Object(m)));
        }
        let (rest, key) = parse_string_literal(input)?;
        let rest = skip_whitespace(rest);
        let (rest, _) = parse_char(rest, b':')?;
        let rest = skip_whitespace(rest);
        let (rest, val) = parse_json_value(rest)?;
        let rest = skip_whitespace(rest);
        input = rest;
        if let Some((rest, _)) = parse_char(rest, b',') {
            input = rest;
        }
        input = skip_whitespace(input);
        m.push((key, val));
    }
    None
}

fn parse_number(input: &[u8]) -> Option<(&[u8], JsonValue)> {
    let mut num_str = String::new();
    for (i, c) in input.iter().enumerate() {
        if c.is_ascii_digit() {
            num_str.push(*c as char);
        } else if num_str.is_empty() {
            return None;
        } else {
            return Some((&input[i..], JsonValue::Number(num_str.parse().unwrap())));
        }
    }
    if num_str.is_empty() {
        None
    } else {
        Some((&[], JsonValue::Number(num_str.parse().unwrap())))
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

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
        assert_eq!(
            parse_bool(b"true"),
            Some(("".as_bytes(), JsonValue::Bool(true)))
        );
        assert_eq!(
            parse_bool(b"false"),
            Some(("".as_bytes(), JsonValue::Bool(false)))
        );
        assert_eq!(parse_bool(b"null"), None);
    }

    #[test]
    fn test_parse_string_literal() {
        assert_eq!(
            parse_json_string_literal(b"\"hello world123\": true"),
            Some((
                ": true".as_bytes(),
                JsonValue::String("hello world123".to_string())
            ))
        );
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(
            parse_number(b"12345"),
            Some(("".as_bytes(), JsonValue::Number(12345)))
        );
    }

    #[test]
    fn test_parse_json_object() {
        assert_eq!(
            parse_object(
                r#"{
        "test": false,
        "friends": [1, 2, true],
        "hi": {
            "another": "one"
        }
    }"#
                .as_bytes(),
            ),
            Some((
                "".as_bytes(),
                JsonValue::Object(vec![
                    ("test".to_string(), JsonValue::Bool(false)),
                    (
                        "friends".to_string(),
                        JsonValue::Array(vec![
                            JsonValue::Number(1),
                            JsonValue::Number(2),
                            JsonValue::Bool(true)
                        ])
                    ),
                    (
                        "hi".to_string(),
                        JsonValue::Object(vec![(
                            "another".to_string(),
                            JsonValue::String("one".to_string())
                        )])
                    )
                ])
            ))
        )
    }

    #[test]
    fn test_parse_json_array() {
        assert_eq!(
            parse_array(b"[ \"hello\" , true , null ]"),
            Some((
                "".as_bytes(),
                JsonValue::Array(vec![
                    JsonValue::String("hello".to_string()),
                    JsonValue::Bool(true),
                    JsonValue::Null
                ])
            ))
        );

        let input = r#"["hello",true,null, [1, 2, 3, true, [false] ]]"#;
        let res = parse_array(input.as_bytes());
        assert_eq!(
            res,
            Some((
                "".as_bytes(),
                JsonValue::Array(vec![
                    JsonValue::String("hello".to_string()),
                    JsonValue::Bool(true),
                    JsonValue::Null,
                    JsonValue::Array(vec![
                        JsonValue::Number(1),
                        JsonValue::Number(2),
                        JsonValue::Number(3),
                        JsonValue::Bool(true),
                        JsonValue::Array(vec![JsonValue::Bool(false)])
                    ])
                ])
            ))
        )
    }
}
