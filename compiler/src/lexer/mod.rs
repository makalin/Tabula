use anyhow::{Context, Result};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Tab,
    Space,
    Newline,
    Word(String),
    Number(i64),
    Float(f64),
    String(String),
    Eof,
}

#[derive(Debug, Clone)]
pub struct TokenWithPos {
    pub token: Token,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer {
    source: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            source: Vec::new(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&self, source: &str) -> Result<Vec<TokenWithPos>> {
        let mut lexer = Self {
            source: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        };

        let mut tokens = Vec::new();

        while !lexer.is_at_end() {
            let start_line = lexer.line;
            let start_column = lexer.column;

            if lexer.peek() == '\t' {
                lexer.advance();
                tokens.push(TokenWithPos {
                    token: Token::Tab,
                    line: start_line,
                    column: start_column,
                });
            } else if lexer.peek() == ' ' {
                lexer.advance();
                tokens.push(TokenWithPos {
                    token: Token::Space,
                    line: start_line,
                    column: start_column,
                });
            } else if lexer.peek() == '\n' {
                lexer.advance();
                tokens.push(TokenWithPos {
                    token: Token::Newline,
                    line: start_line,
                    column: start_column,
                });
            } else if lexer.peek().is_alphabetic() || lexer.peek() == '_' {
                let word = lexer.scan_word();
                tokens.push(TokenWithPos {
                    token: Token::Word(word),
                    line: start_line,
                    column: start_column,
                });
            } else if lexer.peek().is_ascii_digit() {
                let (num, is_float) = lexer.scan_number();
                tokens.push(TokenWithPos {
                    token: if is_float {
                        Token::Float(num as f64)
                    } else {
                        Token::Number(num)
                    },
                    line: start_line,
                    column: start_column,
                });
            } else if lexer.peek() == '"' {
                let string = lexer.scan_string()
                    .with_context(|| format!("Unterminated string at line {}", start_line))?;
                tokens.push(TokenWithPos {
                    token: Token::String(string),
                    line: start_line,
                    column: start_column,
                });
            } else {
                return Err(anyhow::anyhow!(
                    "Unexpected character '{}' at line {}:{}",
                    lexer.peek(),
                    start_line,
                    start_column
                ));
            }
        }

        tokens.push(TokenWithPos {
            token: Token::Eof,
            line: lexer.line,
            column: lexer.column,
        });

        Ok(tokens)
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.position]
        }
    }

    fn advance(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            let ch = self.source[self.position];
            self.position += 1;

            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }

            ch
        }
    }

    fn scan_word(&mut self) -> String {
        let mut word = String::new();
        while !self.is_at_end()
            && (self.peek().is_alphanumeric() || self.peek() == '_')
        {
            word.push(self.advance());
        }
        word
    }

    fn scan_number(&mut self) -> (i64, bool) {
        let mut num_str = String::new();
        let mut is_float = false;

        while !self.is_at_end() && (self.peek().is_ascii_digit() || self.peek() == '.') {
            if self.peek() == '.' {
                if is_float {
                    break;
                }
                is_float = true;
            }
            num_str.push(self.advance());
        }

        if is_float {
            let num = num_str.parse::<f64>().unwrap_or(0.0);
            (num as i64, true)
        } else {
            (num_str.parse::<i64>().unwrap_or(0), false)
        }
    }

    fn scan_string(&mut self) -> Result<String> {
        assert_eq!(self.advance(), '"');
        let mut string = String::new();

        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\\' {
                self.advance();
                match self.peek() {
                    'n' => {
                        string.push('\n');
                        self.advance();
                    }
                    't' => {
                        string.push('\t');
                        self.advance();
                    }
                    '\\' => {
                        string.push('\\');
                        self.advance();
                    }
                    '"' => {
                        string.push('"');
                        self.advance();
                    }
                    _ => {
                        string.push('\\');
                        string.push(self.advance());
                    }
                }
            } else {
                string.push(self.advance());
            }
        }

        if self.is_at_end() {
            return Err(anyhow::anyhow!("Unterminated string"));
        }

        self.advance(); // consume closing quote
        Ok(string)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Tab => write!(f, "TAB"),
            Token::Space => write!(f, "SPACE"),
            Token::Newline => write!(f, "NEWLINE"),
            Token::Word(w) => write!(f, "WORD({})", w),
            Token::Number(n) => write!(f, "NUMBER({})", n),
            Token::Float(n) => write!(f, "FLOAT({})", n),
            Token::String(s) => write!(f, "STRING({:?})", s),
            Token::Eof => write!(f, "EOF"),
        }
    }
}

