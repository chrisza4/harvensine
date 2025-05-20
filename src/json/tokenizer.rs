use core::str;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

pub enum JsonSign {
    NoSign,
    Plus,
    Minus,
}

#[derive(PartialEq, Eq, Debug)]
pub enum JsonValue {
    Object,
    Array,
    String,
    Number,
    TrueValue,
    FalseValue,
    NullValue,
}

#[derive(PartialEq, Eq, Debug)]
pub enum TokenizedError {
    Invalid,
}

pub fn tokenized<R>(mut reader: R) -> Result<JsonValue, TokenizedError>
where
    R: BufRead,
{
    let mut buffer = [0; 1];
    let Ok(_) = reader.read(&mut buffer) else {
        return Err(TokenizedError::Invalid);
    };
    let char = buffer[0] as char;
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
        },
        't' => {
            let mut buf = [0; 3];
            let Ok(_) = reader.read(&mut buf) else {
                return Err(TokenizedError::Invalid);
            };
            match str::from_utf8_mut(&mut buf) {
                Ok(str) if str == "rue" => Ok(JsonValue::TrueValue),
                _ => Err(TokenizedError::Invalid),
            }
        },
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
        _ => Err(TokenizedError::Invalid),
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use crate::json::tokenizer::{tokenized, JsonValue, TokenizedError};

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
}
