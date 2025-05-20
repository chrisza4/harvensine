use core::str;
use std::{collections::HashMap, io::BufRead};

#[derive(PartialEq, Debug)]
pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    TrueValue,
    FalseValue,
    NullValue,
}

#[derive(PartialEq, Eq, Debug)]
pub enum TokenizedError {
    InvalidChar,
    Invalid,
    EndOfString,
}

fn read_one_char<R>(reader: &mut R) -> Result<char, TokenizedError>
where
    R: BufRead,
{
    let mut buffer = [0; 1];
    let Ok(_) = reader.read(&mut buffer) else {
        return Err(TokenizedError::Invalid);
    };
    Ok(buffer[0] as char)
}

fn is_char_in_number(c: &char) -> bool {
    c.is_ascii_digit() || *c == '.' || *c == 'E' || *c == 'e'
}

macro_rules! tokenized {
    ($a: expr, $b: expr) => {
        tokenized($a, $b)
    };
    ($a: expr) => {
        tokenized($a, None)
    };
}

#[derive(PartialEq, Debug)]
pub struct TokenizedResult {
    last_char_read: Option<char>,
    result: JsonValue,
}

fn read_number<R>(reader: &mut R, result: &mut String) -> Result<char, TokenizedError>
where
    R: BufRead,
{
    loop {
        let next_char = read_and_tokenized_char(reader)?;
        if !is_char_in_number(&next_char) {
            return Ok(next_char);
        }
        result.push(next_char);
    }
}

fn read_until_not_space<R>(reader: &mut R) -> Result<char, TokenizedError>
where
    R: BufRead,
{
    let mut char = read_one_char(reader)?;
    loop {
        if char != ' ' {
            break;
        }
        char = read_one_char(reader)?;
    }
    Ok(char)
}

#[allow(dead_code)]
pub fn tokenized<R>(
    reader: &mut R,
    last_char: Option<char>,
) -> Result<TokenizedResult, TokenizedError>
where
    R: BufRead,
{
    let char = match last_char {
        Some(c) => c,
        None => read_until_not_space(reader)?,
    };

    match char {
        'n' => {
            let mut buf = [0; 3];
            let Ok(_) = reader.read(&mut buf) else {
                return Err(TokenizedError::Invalid);
            };
            match str::from_utf8_mut(&mut buf) {
                Ok(str) if str == "ull" => Ok(TokenizedResult {
                    last_char_read: None,
                    result: JsonValue::NullValue,
                }),
                _ => Err(TokenizedError::Invalid),
            }
        }
        't' => {
            let mut buf = [0; 3];
            let Ok(_) = reader.read(&mut buf) else {
                return Err(TokenizedError::Invalid);
            };
            match str::from_utf8_mut(&mut buf) {
                Ok(str) if str == "rue" => Ok(TokenizedResult {
                    last_char_read: None,
                    result: JsonValue::TrueValue,
                }),
                _ => Err(TokenizedError::Invalid),
            }
        }
        'f' => {
            let mut buf = [0; 4];
            let Ok(_) = reader.read(&mut buf) else {
                return Err(TokenizedError::Invalid);
            };
            match str::from_utf8_mut(&mut buf) {
                Ok(str) if str == "alse" => Ok(TokenizedResult {
                    last_char_read: None,
                    result: JsonValue::FalseValue,
                }),
                _ => Err(TokenizedError::Invalid),
            }
        }
        '"' => {
            let mut result = String::new();
            loop {
                let next_char = read_and_tokenized_char(reader);
                match next_char {
                    Ok(c) => result.push(c),
                    Err(TokenizedError::EndOfString) => {
                        break;
                    }
                    Err(e) => return Err(e),
                };
            }
            Ok(TokenizedResult {
                last_char_read: None,
                result: JsonValue::String(result),
            })
        }
        c if c.is_ascii_digit() || c == '-' => {
            let mut result = String::new();
            result.push(c);

            let last_char = read_number(reader, &mut result);
            let parsed_result: Result<f64, std::num::ParseFloatError> = result.parse();
            match parsed_result {
                Ok(c) => Ok(TokenizedResult {
                    last_char_read: Some(last_char.unwrap()),
                    result: JsonValue::Number(c),
                }),
                Err(_) => Err(TokenizedError::Invalid),
            }
        }
        '[' => {
            let mut result: Vec<JsonValue> = Vec::new();
            loop {
                let value = tokenized!(reader);
                let Ok(next_token) = value else {
                    println!("Val: {:?}", value);
                    break;
                };
                println!("Token: {:?}", next_token.result);
                result.push(next_token.result);

                let char = match next_token.last_char_read {
                    Some(' ') | None => read_until_not_space(reader),
                    Some(c) => Ok(c),
                };

                match char {
                    Ok(',') => continue,
                    Ok(']') => break,
                    Ok(_) => return Err(TokenizedError::Invalid),
                    Err(e) => return Err(e),
                }
            }
            Ok(TokenizedResult {
                last_char_read: None,
                result: JsonValue::Array(result),
            })
        }
        '{' => {
            let mut result = HashMap::<String, JsonValue>::new();
            loop {
                let Ok(next_char) = read_until_not_space(reader) else {
                    return Err(TokenizedError::Invalid);
                };
                if next_char == '}' {
                    break;
                }
                let Ok(value) = tokenized(reader, Some(next_char)) else {
                    return Err(TokenizedError::Invalid);
                };
                match value.result {
                    JsonValue::String(key) => {
                        let Ok(char) = read_until_not_space(reader) else {
                            return Err(TokenizedError::Invalid);
                        };
                        if char != ':' {
                            return Err(TokenizedError::Invalid);
                        }
                        let Ok(value) = tokenized!(reader) else {
                            return Err(TokenizedError::Invalid);
                        };
                        result.insert(key, value.result);
                        let char = match value.last_char_read {
                            Some(' ') | None => read_until_not_space(reader),
                            Some(c) => Ok(c),
                        }?;
                        if char == '}' {
                            break;
                        }
                        if char != ',' {
                            return Err(TokenizedError::Invalid);
                        }
                    }
                    _ => return Err(TokenizedError::Invalid),
                }
            }
            Ok(TokenizedResult {
                last_char_read: None,
                result: JsonValue::Object(result),
            })
        }
        c => {
            println!("Invalid token: {:?}", c);
            Err(TokenizedError::Invalid)
        }
    }
}

fn read_and_tokenized_char<R>(reader: &mut R) -> Result<char, TokenizedError>
where
    R: BufRead,
{
    let char = read_one_char(reader)?;
    if char == '\"' {
        return Err(TokenizedError::EndOfString);
    }
    if char != '\\' {
        return Ok(char);
    }
    let next_char = read_one_char(reader)?;
    match next_char {
        '"' => Ok('"'),
        '\\' => Ok('\\'),
        '/' => Ok('/'),
        'n' => Ok('\n'),
        'f' => Ok('\u{000C}'),
        'r' => Ok('\u{000D}'),
        't' => Ok('\u{0009}'),
        'u' => {
            let mut buf = [0; 4];
            let Ok(_) = reader.read(&mut buf) else {
                return Err(TokenizedError::Invalid);
            };
            let Ok(unicode_hex_str) = str::from_utf8_mut(&mut buf) else {
                return Err(TokenizedError::InvalidChar);
            };
            let Ok(unicode_hex) = u32::from_str_radix(&unicode_hex_str, 16) else {
                return Err(TokenizedError::InvalidChar);
            };
            let Some(c) = char::from_u32(unicode_hex) else {
                return Err(TokenizedError::InvalidChar);
            };
            Ok(c)
        }
        _ => Err(TokenizedError::InvalidChar),
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use rstest::rstest;

    use crate::json::tokenizer::*;

    fn buf_reader_from_str(str: &str) -> BufReader<Cursor<&str>> {
        let cursor = Cursor::new(str);
        BufReader::new(cursor)
    }

    #[test]
    pub fn test_tokenized_null() {
        let input = "null";
        let mut reader = buf_reader_from_str(input);

        assert_eq!(
            JsonValue::NullValue,
            tokenized!(&mut reader).unwrap().result
        );
    }

    #[test]
    pub fn test_tokenized_with_spaces_null() {
        let input = "   null";
        let mut reader = buf_reader_from_str(input);

        assert_eq!(
            JsonValue::NullValue,
            tokenized!(&mut reader).unwrap().result
        );
    }

    #[test]
    pub fn test_tokenized_true() {
        let input = "true";
        let mut reader = buf_reader_from_str(input);

        assert_eq!(
            JsonValue::TrueValue,
            tokenized!(&mut reader).unwrap().result
        );
    }

    #[test]
    pub fn test_tokenized_false() {
        let input = "false";
        let mut reader = buf_reader_from_str(input);

        assert_eq!(
            JsonValue::FalseValue,
            tokenized!(&mut reader).unwrap().result
        );
    }

    #[test]
    pub fn test_tokenized_array() {
        let input = "[1, 2, \"haha\", \"hoho\", true]";
        let mut reader = buf_reader_from_str(input);

        assert_eq!(
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
                JsonValue::String(String::from("haha")),
                JsonValue::String(String::from("hoho")),
                JsonValue::TrueValue
            ]),
            tokenized!(&mut reader).unwrap().result
        );
    }

    #[test]
    pub fn test_tokenized_empty_array() {
        let input = "[ ]";
        let mut reader = buf_reader_from_str(input);

        assert_eq!(
            JsonValue::Array(vec![]),
            tokenized!(&mut reader).unwrap().result
        );
    }

    #[test]
    pub fn test_tokenized_nested_array() {
        let input = "[1, 2, \"haha\", [\"f\", \"w\", 3], 4]";
        let mut reader = buf_reader_from_str(input);

        assert_eq!(
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
                JsonValue::String(String::from("haha")),
                JsonValue::Array(vec![
                    JsonValue::String(String::from("f")),
                    JsonValue::String(String::from("w")),
                    JsonValue::Number(3.0),
                ]),
                JsonValue::Number(4.0),
            ]),
            tokenized!(&mut reader).unwrap().result
        );
    }

    #[test]
    pub fn test_tokenized_object() {
        let input = "{ \"ok\": true, \"message\": \"haha\", \"code\": 333}";
        let mut reader = buf_reader_from_str(input);

        assert_eq!(
            JsonValue::Object(HashMap::from([
                ("ok".to_string(), JsonValue::TrueValue),
                ("message".to_string(), JsonValue::String("haha".to_string())),
                ("code".to_string(), JsonValue::Number(333.0))
            ])),
            tokenized!(&mut reader).unwrap().result
        );
    }

    #[test]
    pub fn test_tokenized_empty_object() {
        let input = "{}";
        let mut reader = buf_reader_from_str(input);

        assert_eq!(
            JsonValue::Object(HashMap::from([])),
            tokenized!(&mut reader).unwrap().result
        );
    }
    #[test]
    pub fn test_tokenized_nested_object() {
        let input = "{ \"ok\": true, \"message\": \"haha\", \"data\": {\"a\":  \"b\"} }";
        let mut reader = buf_reader_from_str(input);

        assert_eq!(
            JsonValue::Object(HashMap::from([
                ("ok".to_string(), JsonValue::TrueValue),
                ("message".to_string(), JsonValue::String("haha".to_string())),
                (
                    "data".to_string(),
                    JsonValue::Object(HashMap::from([(
                        "a".to_string(),
                        JsonValue::String("b".to_string())
                    )]))
                )
            ])),
            tokenized!(&mut reader).unwrap().result
        );
    }

    #[test]
    pub fn test_tokenized_err() {
        let input = "nxll";
        let mut reader = buf_reader_from_str(input);

        assert_eq!(Err(TokenizedError::Invalid), tokenized!(&mut reader));
    }

    #[test]
    pub fn test_tokenized_string() {
        let input = "\"hello world\"";
        let mut reader = buf_reader_from_str(input);

        assert_eq!(
            JsonValue::String(String::from("hello world")),
            tokenized!(&mut reader).unwrap().result
        );
    }

    #[test]
    pub fn test_tokenized_number_integer() {
        let input = "123451";
        let mut reader = buf_reader_from_str(input);

        assert_eq!(
            JsonValue::Number(123451.0),
            tokenized!(&mut reader).unwrap().result
        );
    }

    #[test]
    pub fn test_tokenized_number_float() {
        let input = "123.451";
        let mut reader = buf_reader_from_str(input);

        assert_eq!(
            JsonValue::Number(123.451),
            tokenized!(&mut reader).unwrap().result
        );
    }

    #[rstest]
    #[case("123e3", 123000.0)]
    #[case("1E4", 10000.0)]
    pub fn test_tokenized_number_exponent(#[case] input: &str, #[case] expected: f64) {
        let mut reader = buf_reader_from_str(input);

        assert_eq!(
            JsonValue::Number(expected),
            tokenized!(&mut reader).unwrap().result
        );
    }

    #[test]
    pub fn test_tokenized_number_minus() {
        let input = "-123.451";
        let mut reader = buf_reader_from_str(input);

        assert_eq!(
            JsonValue::Number(-123.451),
            tokenized!(&mut reader).unwrap().result
        );
    }

    #[test]
    pub fn test_tokenized_char_normal_char() {
        let input = "multiple_char";
        let mut reader = buf_reader_from_str(input);

        assert_eq!(Ok('m'), read_and_tokenized_char(&mut reader));
        assert_eq!(Ok('u'), read_and_tokenized_char(&mut reader));
    }

    #[test]
    pub fn test_tokenized_char_end_of_string() {
        let input = "\"";
        let mut reader = buf_reader_from_str(input);

        assert_eq!(
            Err(TokenizedError::EndOfString),
            read_and_tokenized_char(&mut reader)
        );
    }

    #[rstest]
    #[case("\\\"something", '"', 's')]
    #[case("\\/something", '/', 's')]
    #[case("\\\\something", '\\', 's')]
    #[case("\\fsomething", '\u{000C}', 's')]
    #[case("\\nsomething", '\n', 's')]
    #[case("\\rsomething", '\u{000D}', 's')]
    #[case("\\tsomething", '\u{0009}', 's')]
    pub fn test_tokenized_char_escaped_char(
        #[case] input: &str,
        #[case] first_char: char,
        #[case] second_char: char,
    ) {
        let mut reader = buf_reader_from_str(input);
        assert_eq!(Ok(first_char), read_and_tokenized_char(&mut reader));
        assert_eq!(Ok(second_char), read_and_tokenized_char(&mut reader));
    }

    #[test]
    pub fn test_tokenized_unicode_char() {
        let input = "\\u004Dx";
        let mut reader = buf_reader_from_str(input);

        assert_eq!(Ok('M'), read_and_tokenized_char(&mut reader));
        assert_eq!(Ok('x'), read_and_tokenized_char(&mut reader));
    }

    #[test]
    pub fn test_tokenized_invalid_escaped() {
        let input = "\\x";
        let mut reader = buf_reader_from_str(input);

        assert_eq!(
            Err(TokenizedError::InvalidChar),
            read_and_tokenized_char(&mut reader)
        );
    }
}
