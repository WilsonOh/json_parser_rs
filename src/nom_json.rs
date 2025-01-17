use nom::{
    branch::alt,
    bytes::{complete::tag, streaming::take_while},
    character::complete::{char, multispace0, one_of},
    combinator::{map, map_res, recognize, value},
    error::ParseError,
    multi::{many0, many1, separated_list0},
    number::complete::recognize_float,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum Number {
    Float(f32),
    Int(i32),
}

#[derive(Debug, PartialEq)]
pub enum JsonValue<'a> {
    Bool(bool),
    String(&'a str),
    Number(Number),
    Null,
    Array(Vec<JsonValue<'a>>),
    Object(Vec<(&'a str, JsonValue<'a>)>),
}

type JsonResult<'a> = IResult<&'a str, JsonValue<'a>>;

fn remove_whitespace<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn parse_json_null(input: &str) -> JsonResult {
    map(tag("null"), |_| JsonValue::Null)(input)
}

fn parse_json_bool(input: &str) -> JsonResult {
    map(
        alt((value(false, tag("false")), value(true, tag("true")))),
        JsonValue::Bool,
    )(input)
}

fn parse_string_literal(input: &str) -> IResult<&str, &str> {
    delimited(tag("\""), take_while(|c| c != '\"'), tag("\""))(input)
}

fn parse_json_string(input: &str) -> JsonResult {
    map(parse_string_literal, JsonValue::String)(input)
}

fn recognize_integer(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

fn parse_float(input: &str) -> JsonResult {
    map(map_res(recognize_float, |s: &str| s.parse::<f32>()), |n| {
        JsonValue::Number(Number::Float(n))
    })(input)
}

fn parse_integer(input: &str) -> JsonResult {
    map(
        map_res(recognize_integer, |s: &str| s.parse::<i32>()),
        |n| JsonValue::Number(Number::Int(n)),
    )(input)
}

fn parse_json_array(input: &str) -> JsonResult {
    map(
        delimited(
            remove_whitespace(tag("[")),
            separated_list0(
                remove_whitespace(tag(",")),
                remove_whitespace(parse_json_value),
            ),
            remove_whitespace(tag("]")),
        ),
        JsonValue::Array,
    )(input)
}

fn parse_json_object(input: &str) -> JsonResult {
    let kv_parser = separated_pair(
        remove_whitespace(parse_string_literal),
        remove_whitespace(tag(":")),
        remove_whitespace(parse_json_value),
    );
    map(
        delimited(
            remove_whitespace(tag("{")),
            separated_list0(remove_whitespace(tag(",")), kv_parser),
            remove_whitespace(tag("}")),
        ),
        JsonValue::Object,
    )(input)
}

fn parse_json_value(input: &str) -> JsonResult {
    alt((
        parse_json_object,
        parse_json_array,
        parse_float,
        parse_integer,
        parse_json_string,
        parse_json_null,
        parse_json_bool,
    ))(input)
}

pub fn parse_json(input: &str) -> Option<JsonValue> {
    parse_json_value(input).ok().map(|(_, val)| val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number_test() {
        assert_eq!(
            parse_float("1.23"),
            Ok(("", JsonValue::Number(Number::Float(1.23))))
        );
        assert_eq!(
            parse_float("1e8"),
            Ok(("", JsonValue::Number(Number::Float(1e8))))
        );
        assert_eq!(
            parse_float("123"),
            Ok(("", JsonValue::Number(Number::Float(123.))))
        );
        assert_eq!(
            parse_integer("123"),
            Ok(("", JsonValue::Number(Number::Int(123))))
        );
    }

    #[test]
    fn parse_null_test() {
        assert_eq!(parse_json_null("null"), Ok(("", JsonValue::Null)));
        assert_eq!(parse_json_null("nullable"), Ok(("able", JsonValue::Null)));
        assert!(parse_json_null("ullable").is_err());
        assert!(parse_json_null("").is_err());
        assert!(parse_json_null("testnull").is_err());
    }

    #[test]
    fn parse_bool_test() {
        assert_eq!(parse_json_bool("true"), Ok(("", JsonValue::Bool(true))));
        assert_eq!(parse_json_bool("false"), Ok(("", JsonValue::Bool(false))));
        assert_eq!(parse_json_bool("falsey"), Ok(("y", JsonValue::Bool(false))));
        assert!(parse_json_bool("hello").is_err());
    }

    #[test]
    fn parse_json_object_test() {
        let v = JsonValue::Object(vec![
            ("test", JsonValue::String("foo")),
            (
                "bar",
                JsonValue::Object(vec![("test", JsonValue::String("test"))]),
            ),
            (
                "friends",
                JsonValue::Array(vec![
                    JsonValue::Number(Number::Float(123.0)),
                    JsonValue::Number(Number::Float(1.23)),
                    JsonValue::Object(vec![("bar", JsonValue::String("baz"))]),
                ]),
            ),
            (
                "another",
                JsonValue::Object(vec![
                    ("one", JsonValue::Array(vec![JsonValue::Null])),
                    ("two", JsonValue::Bool(false)),
                ]),
            ),
        ]);
        assert_eq!(
            parse_json_object(
                r#"{
                    "test": "foo",
                    "bar": {
                        "test": "test"
                    },

                    "friends":     [123, 1.23, {
                        "bar": "baz"
                    }],

                    "another": {
                        "one": [null  ],
                        "two": false
                    }
            }"#
            ),
            Ok(("", v))
        );
    }

    #[test]
    fn parse_json_array_test() {
        assert_eq!(parse_json_array("[]"), Ok(("", JsonValue::Array(vec![]))));
        assert_eq!(
            parse_json_array("[    123 , 1.23, true, \"hello world\",  [   false  ]   ]",),
            Ok((
                "",
                JsonValue::Array(vec![
                    JsonValue::Number(Number::Float(123.0)),
                    JsonValue::Number(Number::Float(1.23)),
                    JsonValue::Bool(true),
                    JsonValue::String("hello world"),
                    JsonValue::Array(vec![JsonValue::Bool(false)])
                ])
            ))
        )
    }

    #[test]
    fn parse_string_literal_test() {
        assert_eq!(
            parse_json_string("\"hello world 123\"true"),
            Ok(("true", JsonValue::String("hello world 123")))
        );
        assert!(parse_string_literal("\"hello world 123true").is_err());
    }
}
