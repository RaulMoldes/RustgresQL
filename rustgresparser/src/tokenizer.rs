// Generates sql tokens from the input string.
use rustgresparser::keywords::{all_delimiters, all_keywords, all_symbols};
use std::str::Chars;

#[derive(Debug, PartialEq)]
enum Token {
    Keyword(String),
    Identifier(String),
    Literal(String),
    Symbol(char),
    Whitespace,
    Unknown(char),
}

struct Tokenizer<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
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

    fn tokenize(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.current_char {
            match c {
                // Whitespace
                c if c.is_whitespace() => {
                    self.advance();
                    tokens.push(Token::Whitespace);
                }
                // Symbols
                c if all_symbols().contains(&c) => {
                    tokens.push(Token::Symbol(c));
                    self.advance();
                }

                // String literals
                c if all_delimiters().contains(&c) => {
                    tokens.push(self.consume_literal(c));
                }
                // Keywords and identifiers
                c if c.is_alphabetic() => {
                    tokens.push(self.consume_keyword_or_identifier());
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

        Token::Literal(literal)
    }

    fn consume_keyword_or_identifier(&mut self) -> Token {
        let mut word = String::new();

        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                word.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if is_keyword(&word) {
            Token::Keyword(word)
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

fn is_keyword(word: &str) -> bool {
    all_keywords().contains(&word.to_lowercase())
}

fn main() {
    let sql_query = "SELECT name, age, CASE WHEN age < 6 THEN bebe ELSE mayor END as tipo FROM users WHERE age > 30;";
    let tokenizer = Tokenizer::new(sql_query);
    let tokens = tokenizer.tokenize();

    for token in tokens {
        println!("{:?}", token);
    }
}
