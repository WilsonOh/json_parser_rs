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
