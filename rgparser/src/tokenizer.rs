// src/tokenizer.rs
// Generates sql tokens from the input string.
use crate::keywords::{SQL_KEYWORDS, SQL_FUNCTIONS, SQL_DATATYPES, SQL_ENTITIES, LITERAL_DELIMITERS, OPERATORS, PUNCTUATION};
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),
    Datatype(String),
    Function(String),
    Identifier(String),
    Entity(String),
    Literal(String),
    Punctuation(char),
    Operator(char),
    Whitespace,
    Unknown(char),
}

impl Token {
    pub fn is_keyword(&self) -> bool {
        matches!(self, Token::Keyword(_))
    }

    pub fn is_datatype(&self) -> bool {
        matches!(self, Token::Datatype(_))
    }

    pub fn is_function(&self) -> bool {
        matches!(self, Token::Function(_))
    }
    pub fn is_identifier(&self) -> bool {
        matches!(self, Token::Identifier(_))
    }
    pub fn is_literal(&self) -> bool {
        matches!(self, Token::Literal(_))
    }
    pub fn is_punctuation(&self) -> bool {
        matches!(self, Token::Punctuation(_))
    }
    pub fn is_operator(&self) -> bool {
        matches!(self, Token::Operator(_))
    }
    pub fn is_whitespace(&self) -> bool {
        matches!(self, Token::Whitespace)
    }
    pub fn is_unknown(&self) -> bool {
        matches!(self, Token::Unknown(_))
    }
    pub fn is_entity(&self) -> bool {
        matches!(self, Token::Entity(_))
    }
    pub fn get_value(&self) -> String {
        match self {
            Token::Datatype(s) => s.clone(),
            Token::Function(s) => s.clone(),
            Token::Keyword(s) => s.clone(),
            Token::Identifier(s) => s.clone(),
            Token::Literal(s) => s.clone(),
            Token::Punctuation(c) => c.to_string(),
            Token::Operator(c) => c.to_string(),
            Token::Whitespace => " ".to_string(),
            Token::Unknown(c) => c.to_string(),
            Token::Entity(s) => s.clone(),
        }
    }

    
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Datatype(s) => write!(f, "Datatype({})", s),
            Token::Function(s) => write!(f, "Function({})", s),
            Token::Keyword(s) => write!(f, "Keyword({})", s),
            Token::Identifier(s) => write!(f, "Identifier({})", s),
            Token::Literal(s) => write!(f, "Literal({})", s),
            Token::Punctuation(c) => write!(f, "Punctuation({})", c),
            Token::Operator(c) => write!(f, "Operator({})", c),
            Token::Entity(s) => write!(f, "Entity({})", s),
            Token::Whitespace => write!(f, "Whitespace"),
            Token::Unknown(c) => write!(f, "Unknown({})", c),
        }
    }
}
pub struct Tokenizer<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut tokenizer = Tokenizer {
            input: input.chars(),
            current_char: None,
        };
        tokenizer.advance();
        tokenizer
    }

    fn advance(&mut self) {
        self.current_char = self.input.next();
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.current_char {
            match c {
                // Whitespace
                c if c.is_whitespace() => {
                    self.advance();
                    tokens.push(Token::Whitespace);
                }
                // Symbols
                c if PUNCTUATION.contains(&c) => {
                    tokens.push(Token::Punctuation(c));
                    self.advance();
                }

                // Operators
                c if OPERATORS.contains(&c) => {
                    tokens.push(Token::Operator(c));
                    self.advance();
                }

                // String literals
                c if LITERAL_DELIMITERS.contains(&c) => {
                    tokens.push(self.consume_literal(c));
                }
                // Keywords and identifiers
                c if c.is_alphabetic() => {
                    tokens.push(self.consume_word());
                }
                // Numbers (literals)
                c if c.is_numeric() => {
                    tokens.push(self.consume_number());
                }
                // Unknown or unexpected characters
                _ => {
                    tokens.push(Token::Unknown(c));
                    self.advance();
                }
            }
        }

        tokens
    }

    fn consume_literal(&mut self, delimiter: char) -> Token {
        let mut literal = String::new();
        literal.push(delimiter);
        self.advance();

        while let Some(c) = self.current_char {
            literal.push(c);
            self.advance();

            if c == delimiter {
                break;
            }
        }

        if self.current_char.is_none() && !literal.ends_with(delimiter) {
            // Handle the case where the literal is not properly closed
            return Token::Unknown(literal.chars().next().unwrap());
        }

        Token::Literal(literal)
    }

    fn consume_word(&mut self) -> Token {
        let mut word = String::new();

        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                word.push(c);
                self.advance();
            } else {
                break;
            }
        }
     
        if SQL_KEYWORDS.contains(&word.to_lowercase().as_str()){
            Token::Keyword(word)
        } else if SQL_FUNCTIONS.contains(&word.to_lowercase().as_str()){
            Token::Function(word)
        } else if SQL_DATATYPES.contains(&word.to_lowercase().as_str()){
            Token::Datatype(word)
        } else if SQL_ENTITIES.contains(&word.to_lowercase().as_str()) {
            Token::Entity(word)
        } else {
            Token::Identifier(word)
        }
    }

    fn consume_number(&mut self) -> Token {
        let mut number = String::new();

        while let Some(c) = self.current_char {
            if c.is_numeric() || c == '.' {
                number.push(c);
                self.advance();
            } else {
                break;
            }
        }

        Token::Literal(number)
    }
}


   
