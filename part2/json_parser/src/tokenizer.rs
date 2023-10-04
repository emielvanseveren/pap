#[derive(Debug)]
pub enum JsonToken<'a> {
    BeginArray,      // [
    EndArray,        // ]
    BeginObject,     // {
    EndObject,       // }
    String(&'a str), // "..."
    Number(f64),     // 123.45
    Boolean(bool),   // true | false
    Null,            // null
    Colon,           // :
    Comma,           // ,
}

pub struct Tokenizer<'a> {
    pub input: &'a [u8],
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self { input }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = std::result::Result<JsonToken<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip whitespaces
        while let Some(&byte) = self.input.first() {
            if !byte.is_ascii_whitespace() {
                break;
            }
            self.input = &self.input[1..];
        }

        match self.input.first() {
            Some(b'{') => {
                self.input = &self.input[1..];
                Some(Ok(JsonToken::BeginObject))
            }
            Some(b'}') => {
                self.input = &self.input[1..];
                Some(Ok(JsonToken::EndObject))
            }
            Some(b'[') => {
                self.input = &self.input[1..];
                Some(Ok(JsonToken::BeginArray))
            }
            Some(b']') => {
                self.input = &self.input[1..];
                Some(Ok(JsonToken::EndArray))
            }
            Some(b':') => {
                self.input = &self.input[1..];
                Some(Ok(JsonToken::Colon))
            }
            Some(b',') => {
                self.input = &self.input[1..];
                Some(Ok(JsonToken::Comma))
            }
            Some(b'"') => {
                // continue iterator until we find the end of the string
                if let Some(end) = self.input[1..].iter().position(|&b| b == b'"') {
                    let s = std::str::from_utf8(&self.input[1..1 + end])
                        .expect("Invalid UTF-8 sequence");

                    // set the input to the remaining bytes
                    self.input = &self.input[end + 2..];
                    Some(Ok(JsonToken::String(s)))
                } else {
                    Some(Err("Unclosed string".to_string()))
                }
            }
            Some(b't') if self.input.starts_with(b"true") => {
                self.input = &self.input[4..];
                Some(Ok(JsonToken::Boolean(true)))
            }
            Some(b'f') if self.input.starts_with(b"false") => {
                self.input = &self.input[5..];
                Some(Ok(JsonToken::Boolean(false)))
            }
            Some(b'n') if self.input.starts_with(b"null") => {
                self.input = &self.input[4..];
                Some(Ok(JsonToken::Null))
            }
            Some(&ch) if ch.is_ascii_digit() || ch == b'-' => {
                // Capturing a number, this includes scientific notation e.g. 3.14e-2
                let end = self
                    .input
                    .iter()
                    .position(|&b| {
                        !b.is_ascii_digit() && b != b'.' && b != b'-' && b != b'e' && b != b'E'
                    })
                    .unwrap_or(self.input.len());
                let num_str = String::from_utf8(self.input[..end].to_vec())
                    .map_err(|_| "Invalid UTF-8 sequence".to_string())
                    .unwrap();
                let num = num_str
                    .parse::<f64>()
                    .map_err(|_| "Invalid number format".to_string())
                    .unwrap();
                self.input = &self.input[end..];
                Some(Ok(JsonToken::Number(num)))
            }
            // string representation
            Some(_) => Some(Err(format!("Unknown token: {}", self.input[0] as char))),
            None => None, // End of input
        }
    }
}
