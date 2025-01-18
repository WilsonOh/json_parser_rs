use crate::{JsonValue, Number};

pub fn format_json(
    json: &JsonValue,
    _num_spaces: usize,
    _indent_level: usize,
    _compact: bool,
) -> String {
    match json {
        JsonValue::Bool(b) => bool::to_string(b),
        JsonValue::String(s) => "\"".to_string() + s + "\"",
        JsonValue::Null => "null".to_string(),
        JsonValue::Number(n) => match n {
            Number::Int(i) => i.to_string(),
            Number::Float(f) => f.to_string(),
        },
        JsonValue::Array(a) => {
            let items = a
                .iter()
                .map(|j| format_json(j, _num_spaces, _indent_level + 1, _compact))
                .collect::<Vec<_>>();
            if _compact {
                return format!("[{}]", items.join(","));
            }
            let indent = " ".repeat(_num_spaces * _indent_level);
            let closing_indent = " ".repeat(_num_spaces * (_indent_level - 1));
            let items_string = items.join(&format!(",\n{indent}"));
            "[\n".to_string() + &indent + &items_string + &format!("\n{closing_indent}]")
        }
        JsonValue::Object(o) => {
            let items = o
                .iter()
                .map(|(k, v)| {
                    ("\"".to_owned() + k + "\"")
                        + if _compact { ":" } else { ": " }
                        + &format_json(v, _num_spaces, _indent_level + 1, _compact)
                })
                .collect::<Vec<_>>();
            if _compact {
                return format!("{{{}}}", items.join(","));
            }
            let indent = " ".repeat(_num_spaces * _indent_level);
            let closing_indent = " ".repeat(_num_spaces * (_indent_level - 1));
            let items_string = items.join(&format!(",\n{indent}"));
            "{\n".to_string() + &indent + &items_string + &format!("\n{closing_indent}}}")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::nom_json::parse_json;

    use super::format_json;

    #[test]
    fn formatter_test() {
        let input = r#"{"foo":"bar","other":[1,2,3,{"baz":null}]}"#;
        let json = parse_json(input).unwrap();
        let formatted = format_json(&json, 2, 1, false);
        let expected = "{\n  \"foo\": \"bar\",\n  \"other\": [\n    1,\n    2,\n    3,\n    {\n      \"baz\": null\n    }\n  ]\n}";
        assert_eq!(formatted, expected);
    }

    #[test]
    fn formatter_indent_level_test() {
        let input = r#"{"foo":"bar","other":[1,2,3,{"baz":null}]}"#;
        let json = parse_json(input).unwrap();
        let formatted = format_json(&json, 4, 1, false);
        let expected = "{\n    \"foo\": \"bar\",\n    \"other\": [\n        1,\n        2,\n        3,\n        {\n            \"baz\": null\n        }\n    ]\n}";
        assert_eq!(formatted, expected);
    }

    #[test]
    fn formatter_compact_test() {
        let input = "{\n    \"foo\": \"bar\",\n    \"other\": [\n        1,\n        2,\n        3,\n        {\n            \"baz\": null\n        }\n    ]\n}";
        let json = parse_json(input).unwrap();
        let formatted = format_json(&json, 2, 1, true);
        let expected = "{\"foo\":\"bar\",\"other\":[1,2,3,{\"baz\":null}]}";
        assert_eq!(formatted, expected);
    }
}
