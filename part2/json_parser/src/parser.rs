use crate::tokenizer::JsonToken;

#[derive(Debug, PartialEq)]
pub enum JsonValue<'a> {
    Object { kv: Vec<(&'a str, JsonValue<'a>)> },
    Array { values: Vec<JsonValue<'a>> },
    Number(f64),
    String(&'a str),
    Boolean(bool),
    Null,
}

// Parser takes an iterator, which in this case is the tokenizer
pub struct Parser<'a, I>
where
    I: Iterator<Item = std::result::Result<JsonToken<'a>, String>>,
{
    tokens: I,
    current_token: Option<std::result::Result<JsonToken<'a>, String>>,
}

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = std::result::Result<JsonToken<'a>, String>>,
{
    pub fn new(tokens: I) -> Self {
        let mut parser = Parser {
            tokens,
            current_token: None,
        };

        // set the current token to the first token
        parser.advance();
        parser
    }
    pub fn parse_value(&mut self) -> Result<JsonValue<'a>, String> {
        match self.current_token {
            Some(Ok(JsonToken::BeginObject)) => self.parse_object(),
            Some(Ok(JsonToken::BeginArray)) => self.parse_array(),
            Some(Ok(JsonToken::String(s))) => {
                self.advance();
                Ok(JsonValue::String(s))
            }
            Some(Ok(JsonToken::Number(n))) => {
                self.advance();
                Ok(JsonValue::Number(n))
            }
            Some(Ok(JsonToken::Boolean(b))) => {
                self.advance();
                Ok(JsonValue::Boolean(b))
            }
            Some(Ok(JsonToken::Null)) => {
                self.advance();
                Ok(JsonValue::Null)
            }
            _ => Err("Unexpected token".to_string()),
        }
    }

    fn advance(&mut self) {
        self.current_token = self.tokens.next();
    }

    fn parse_object(&mut self) -> Result<JsonValue<'a>, String> {
        if let Some(Ok(JsonToken::BeginObject)) = self.current_token {
            self.advance(); // consume '{'

            let mut kv = Vec::new();
            while let Some(Ok(JsonToken::String(_))) = self.current_token {
                kv.push(self.parse_kv()?);

                // Check if there's a comma to continue parsing
                if let Some(Ok(JsonToken::Comma)) = self.current_token {
                    self.advance(); // consume ','
                    continue;
                } else {
                    break;
                }
            }

            if let Some(Ok(JsonToken::EndObject)) = self.current_token {
                self.advance(); // consume '}'
                Ok(JsonValue::Object { kv })
            } else {
                Err(format!(
                    "Expected '}}' to close object, but found {:?}",
                    self.current_token
                ))
            }
        } else {
            Err("Expected '{' to start an object.".to_string())
        }
    }

    fn parse_kv(&mut self) -> Result<(&'a str, JsonValue<'a>), String> {
        // check if the current token is a string and peek the next token to see if it's a ':'

        if let Some(Ok(JsonToken::String(key))) = self.current_token {
            self.advance(); // consume key

            if let Some(Ok(JsonToken::Colon)) = self.current_token {
                self.advance(); // consume ':'
                let value = self.parse_value()?;
                Ok((key, value))
            } else {
                Err("Expected ':' after object key.".to_string())
            }
        } else {
            Err("Expected string as object key.".to_string())
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue<'a>, String> {
        // consume current token '['
        self.advance();

        let mut values = Vec::new();
        while let Some(Ok(token)) = &self.current_token {
            match token {
                JsonToken::EndArray => {
                    break;
                }
                JsonToken::Comma => {
                    self.advance();
                }
                _ => {
                    values.push(self.parse_value()?);
                }
            }
        }

        if let Some(Ok(JsonToken::EndArray)) = self.current_token {
            self.advance(); // consume ']'
            Ok(JsonValue::Array { values })
        } else {
            Err("Expected ']' to close array.".to_string())
        }
    }
}
