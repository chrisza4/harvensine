use core::str;
use std::io::BufRead;

pub enum JsonSign {
    NoSign,
    Plus,
    Minus,
}

#[derive(PartialEq, Eq, Debug)]
pub enum JsonValue {
    Object,
    Array,
    String(String),
    Number,
    TrueValue,
    FalseValue,
    NullValue,
}

#[derive(PartialEq, Eq, Debug)]
pub enum TokenizedError {
    Invalid,
    EndOfString
}

fn read_one_char<R>(mut reader: R) -> Result<char, TokenizedError>
where
    R: BufRead,
{
    let mut buffer = [0; 1];
    let Ok(_) = reader.read(&mut buffer) else {
        return Err(TokenizedError::Invalid);
    };
    Ok(buffer[0] as char)
}

pub fn tokenized<R>(mut reader: R) -> Result<JsonValue, TokenizedError>
where
    R: BufRead,
{
    let mut char = read_one_char(&mut reader)?;

    loop {
        if char != ' ' {
            break;
        }
        char = read_one_char(&mut reader)?;
    }

    match char {
        'n' => {
            let mut buf = [0; 3];
            let Ok(_) = reader.read(&mut buf) else {
                return Err(TokenizedError::Invalid);
            };
            match str::from_utf8_mut(&mut buf) {
                Ok(str) if str == "ull" => Ok(JsonValue::NullValue),
                _ => Err(TokenizedError::Invalid),
            }
        }
        't' => {
            let mut buf = [0; 3];
            let Ok(_) = reader.read(&mut buf) else {
                return Err(TokenizedError::Invalid);
            };
            match str::from_utf8_mut(&mut buf) {
                Ok(str) if str == "rue" => Ok(JsonValue::TrueValue),
                _ => Err(TokenizedError::Invalid),
            }
        }
        'f' => {
            let mut buf = [0; 4];
            let Ok(_) = reader.read(&mut buf) else {
                return Err(TokenizedError::Invalid);
            };
            match str::from_utf8_mut(&mut buf) {
                Ok(str) if str == "alse" => Ok(JsonValue::FalseValue),
                _ => Err(TokenizedError::Invalid),
            }
        },
        '"' => {
            todo!()
        },
        _ => Err(TokenizedError::Invalid),
    }
}

fn read_and_tokenized_char<R>(mut reader: R) -> Result<char, TokenizedError> 
where
    R: BufRead,
{
    let char = read_one_char(&mut reader)?;
    if char == '\"' {
        return Err(TokenizedError::EndOfString);
    }
    if char != '\\' {
        return Ok(char);
    }
    let next_char = read_one_char(&mut reader)?;
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
                return Err(TokenizedError::Invalid);
            };
            let Ok(unicode_hex) = u32::from_str_radix(&unicode_hex_str, 16) else {
                return Err(TokenizedError::Invalid);
            };
            let Some(c) = char::from_u32(unicode_hex) else {
                return Err(TokenizedError::Invalid);
            };
            Ok(c)
        },
        _ => Err(TokenizedError::Invalid)
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
        let reader = buf_reader_from_str(input);

        assert_eq!(Ok(JsonValue::NullValue), tokenized(reader));
    }

    #[test]
    pub fn test_tokenized_with_spaces_null() {
        let input = "   null";
        let reader = buf_reader_from_str(input);

        assert_eq!(Ok(JsonValue::NullValue), tokenized(reader));
    }

    #[test]
    pub fn test_tokenized_true() {
        let input = "true";
        let reader = buf_reader_from_str(input);

        assert_eq!(Ok(JsonValue::TrueValue), tokenized(reader));
    }

    #[test]
    pub fn test_tokenized_false() {
        let input = "false";
        let reader = buf_reader_from_str(input);

        assert_eq!(Ok(JsonValue::FalseValue), tokenized(reader));
    }

    #[test]
    pub fn test_tokenized_err() {
        let input = "nxll";
        let reader = buf_reader_from_str(input);

        assert_eq!(Err(TokenizedError::Invalid), tokenized(reader));
    }

    #[test]
    #[ignore="Wait"]
    pub fn test_tokenized_string() {
        let input = "\"hello world\"";
        let reader = buf_reader_from_str(input);

        assert_eq!(Ok(JsonValue::String(String::from("hello world"))), tokenized(reader));
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

        assert_eq!(Err(TokenizedError::EndOfString), read_and_tokenized_char(&mut reader));
    }

    #[rstest]
    #[case("\\\"something", '"', 's')]
    #[case("\\/something", '/', 's')]
    #[case("\\\\something", '\\', 's')]
    #[case("\\fsomething", '\u{000C}', 's')]
    #[case("\\nsomething", '\n', 's')]
    #[case("\\rsomething", '\u{000D}', 's')]
    #[case("\\tsomething", '\u{0009}', 's')]
    pub fn test_tokenized_char_escaped_char(#[case] input: &str, #[case] first_char: char, #[case] second_char: char) {
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

        assert_eq!(Err(TokenizedError::Invalid), read_and_tokenized_char(&mut reader));
    }
}
