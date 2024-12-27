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


   


#[cfg(test)]
mod tests {
    use super::*; // Importar el código de tu tokenizador

    // Test de una consulta básica con SELECT
    #[test]
    fn test_select_query() {
        let sql_query = "SELECT name, age FROM users;";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![
            Token::Keyword("SELECT".to_string()),
            Token::Whitespace,
            Token::Identifier("name".to_string()),
            Token::Punctuation(','),
            Token::Whitespace,
            Token::Identifier("age".to_string()),
            Token::Whitespace,
            Token::Keyword("FROM".to_string()),
            Token::Whitespace,
            Token::Identifier("users".to_string()),
            Token::Punctuation(';'),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    // Test con un operador y literales numéricos
    #[test]
    fn test_operator_and_numeric_literals() {
        let sql_query = "SELECT price FROM products WHERE price > 100;";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![
            Token::Keyword("SELECT".to_string()),
            Token::Whitespace,
            Token::Identifier("price".to_string()),
            Token::Whitespace,
            Token::Keyword("FROM".to_string()),
            Token::Whitespace,
            Token::Identifier("products".to_string()),
            Token::Whitespace,
            Token::Keyword("WHERE".to_string()),
            Token::Whitespace,
            Token::Identifier("price".to_string()),
            Token::Whitespace,
            Token::Operator('>'),
            Token::Whitespace,
            Token::Literal("100".to_string()),
            Token::Punctuation(';'),
        ];

        assert_eq!(tokens, expected_tokens);
    }


    // Test con un operador y literales numéricos
    #[test]
    fn test_create_table() {
        let sql_query = "CREATE TABLE users (id INT, name VARCHAR(255));";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![Token::Keyword("CREATE".to_string()), Token::Whitespace, Token::Entity("TABLE".to_string()), Token::Whitespace, Token::Identifier("users".to_string()), Token::Whitespace, Token::Punctuation('('), Token::Identifier("id".to_string()), Token::Whitespace, Token::Datatype("INT".to_string()), Token::Punctuation(','), Token::Whitespace, Token::Identifier("name".to_string()), Token::Whitespace, Token::Datatype("VARCHAR".to_string()), Token::Punctuation('('), Token::Literal("255".to_string()), Token::Punctuation(')'), Token::Punctuation(')'), Token::Punctuation(';')];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_insert_into() {
        let sql_query = "INSERT INTO users (name, age) VALUES ('John', 30);";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![Token::Keyword("INSERT".to_string()), Token::Whitespace, Token::Keyword("INTO".to_string()), Token::Whitespace, Token::Identifier("users".to_string()), Token::Whitespace, Token::Punctuation('('), Token::Identifier("name".to_string()), Token::Punctuation(','), Token::Whitespace, Token::Identifier("age".to_string()), Token::Punctuation(')'), Token::Whitespace, Token::Keyword("VALUES".to_string()), Token::Whitespace, Token::Punctuation('('), Token::Literal("'John'".to_string()), Token::Punctuation(','), Token::Whitespace, Token::Literal("30".to_string()), Token::Punctuation(')'), Token::Punctuation(';')];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_update() {
        let sql_query = "UPDATE users SET age = 31 WHERE name = 'John';";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![Token::Keyword("UPDATE".to_string()), Token::Whitespace, Token::Identifier("users".to_string()), Token::Whitespace, Token::Keyword("SET".to_string()), Token::Whitespace, Token::Identifier("age".to_string()), Token::Whitespace, Token::Operator('='), Token::Whitespace, Token::Literal("31".to_string()), Token::Whitespace, Token::Keyword("WHERE".to_string()), Token::Whitespace, Token::Identifier("name".to_string()), Token::Whitespace, Token::Operator('='), Token::Whitespace, Token::Literal("'John'".to_string()), Token::Punctuation(';')];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_delete() {
        let sql_query = "DELETE FROM users WHERE name = 'John';";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![Token::Keyword("DELETE".to_string()), Token::Whitespace, Token::Keyword("FROM".to_string()), Token::Whitespace, Token::Identifier("users".to_string()), Token::Whitespace, Token::Keyword("WHERE".to_string()), Token::Whitespace, Token::Identifier("name".to_string()), Token::Whitespace, Token::Operator('='), Token::Whitespace, Token::Literal("'John'".to_string()), Token::Punctuation(';')];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_truncate() {
        let sql_query = "TRUNCATE TABLE users;";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![Token::Keyword("TRUNCATE".to_string()), Token::Whitespace, Token::Entity("TABLE".to_string()), Token::Whitespace, Token::Identifier("users".to_string()), Token::Punctuation(';')];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_drop_table() {
        let sql_query = "DROP TABLE users;";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![Token::Keyword("DROP".to_string()), Token::Whitespace, Token::Entity("TABLE".to_string()), Token::Whitespace, Token::Identifier("users".to_string()), Token::Punctuation(';')];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_group_by(){
        let sql_query = "SELECT name, COUNT(*) FROM users GROUP BY name;";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![Token::Keyword("SELECT".to_string()), Token::Whitespace, Token::Identifier("name".to_string()), Token::Punctuation(','), Token::Whitespace, Token::Function("COUNT".to_string()), Token::Punctuation('('), Token::Operator('*'), Token::Punctuation(')'), Token::Whitespace, Token::Keyword("FROM".to_string()), Token::Whitespace, Token::Identifier("users".to_string()), Token::Whitespace, Token::Keyword("GROUP".to_string()), Token::Whitespace, Token::Keyword("BY".to_string()), Token::Whitespace, Token::Identifier("name".to_string()), Token::Punctuation(';')];

        assert_eq!(tokens, expected_tokens);

    }
    // Test con un literal de texto
    #[test]
    fn test_string_literal() {
        let sql_query = "SELECT name FROM users WHERE name = 'John';";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![
            Token::Keyword("SELECT".to_string()),
            Token::Whitespace,
            Token::Identifier("name".to_string()),
            Token::Whitespace,
            Token::Keyword("FROM".to_string()),
            Token::Whitespace,
            Token::Identifier("users".to_string()),
            Token::Whitespace,
            Token::Keyword("WHERE".to_string()),
            Token::Whitespace,
            Token::Identifier("name".to_string()),
            Token::Whitespace,
            Token::Operator('='),
            Token::Whitespace,
            Token::Literal("'John'".to_string()),
            Token::Punctuation(';'),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    // Test con una consulta con una cláusula CASE
    #[test]
    fn test_case_when_query() {
        let sql_query =
            "SELECT name, age, CASE WHEN age < 6 THEN 'bebe' ELSE 'adulto' END AS tipo FROM users;";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![
            Token::Keyword("SELECT".to_string()),
            Token::Whitespace,
            Token::Identifier("name".to_string()),
            Token::Punctuation(','),
            Token::Whitespace,
            Token::Identifier("age".to_string()),
            Token::Punctuation(','),
            Token::Whitespace,
            Token::Keyword("CASE".to_string()),
            Token::Whitespace,
            Token::Keyword("WHEN".to_string()),
            Token::Whitespace,
            Token::Identifier("age".to_string()),
            Token::Whitespace,
            Token::Operator('<'),
            Token::Whitespace,
            Token::Literal("6".to_string()),
            Token::Whitespace,
            Token::Keyword("THEN".to_string()),
            Token::Whitespace,
            Token::Literal("'bebe'".to_string()),
            Token::Whitespace,
            Token::Keyword("ELSE".to_string()),
            Token::Whitespace,
            Token::Literal("'adulto'".to_string()),
            Token::Whitespace,
            Token::Keyword("END".to_string()),
            Token::Whitespace,
            Token::Keyword("AS".to_string()),
            Token::Whitespace,
            Token::Identifier("tipo".to_string()),
            Token::Whitespace,
            Token::Keyword("FROM".to_string()),
            Token::Whitespace,
            Token::Identifier("users".to_string()),
            Token::Punctuation(';'),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    // Test con un número con decimales
    #[test]
    fn test_decimal_number() {
        let sql_query = "SELECT price FROM products WHERE price > 10.5;";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![
            Token::Keyword("SELECT".to_string()),
            Token::Whitespace,
            Token::Identifier("price".to_string()),
            Token::Whitespace,
            Token::Keyword("FROM".to_string()),
            Token::Whitespace,
            Token::Identifier("products".to_string()),
            Token::Whitespace,
            Token::Keyword("WHERE".to_string()),
            Token::Whitespace,
            Token::Identifier("price".to_string()),
            Token::Whitespace,
            Token::Operator('>'),
            Token::Whitespace,
            Token::Literal("10.5".to_string()),
            Token::Punctuation(';'),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    // Test de manejo de un símbolo de puntuación
    #[test]
    fn test_punctuation() {
        let sql_query = "SELECT * FROM users WHERE age > 30;";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![
            Token::Keyword("SELECT".to_string()),
            Token::Whitespace,
            Token::Operator('*'),
            Token::Whitespace,
            Token::Keyword("FROM".to_string()),
            Token::Whitespace,
            Token::Identifier("users".to_string()),
            Token::Whitespace,
            Token::Keyword("WHERE".to_string()),
            Token::Whitespace,
            Token::Identifier("age".to_string()),
            Token::Whitespace,
            Token::Operator('>'),
            Token::Whitespace,
            Token::Literal("30".to_string()),
            Token::Punctuation(';'),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    // Test con una consulta con un identificador con guión bajo
    #[test]
    fn test_identifier_with_underscore() {
        let sql_query = "SELECT first_name, last_name FROM users;";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![
            Token::Keyword("SELECT".to_string()),
            Token::Whitespace,
            Token::Identifier("first_name".to_string()),
            Token::Punctuation(','),
            Token::Whitespace,
            Token::Identifier("last_name".to_string()),
            Token::Whitespace,
            Token::Keyword("FROM".to_string()),
            Token::Whitespace,
            Token::Identifier("users".to_string()),
            Token::Punctuation(';'),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    // Test con un carácter desconocido
    #[test]
    fn test_unknown_character() {
        let sql_query = "SELECT * FROM users # WHERE age > 30;";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![
            Token::Keyword("SELECT".to_string()),
            Token::Whitespace,
            Token::Operator('*'),
            Token::Whitespace,
            Token::Keyword("FROM".to_string()),
            Token::Whitespace,
            Token::Identifier("users".to_string()),
            Token::Whitespace,
            Token::Unknown('#'),
            Token::Whitespace,
            Token::Keyword("WHERE".to_string()),
            Token::Whitespace,
            Token::Identifier("age".to_string()),
            Token::Whitespace,
            Token::Operator('>'),
            Token::Whitespace,
            Token::Literal("30".to_string()),
            Token::Punctuation(';'),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    // Test con una instrucción LIKE
    #[test]
    fn test_like_clause() {
        let sql_query = "SELECT name FROM users WHERE name LIKE 'J%';";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![
            Token::Keyword("SELECT".to_string()),
            Token::Whitespace,
            Token::Identifier("name".to_string()),
            Token::Whitespace,
            Token::Keyword("FROM".to_string()),
            Token::Whitespace,
            Token::Identifier("users".to_string()),
            Token::Whitespace,
            Token::Keyword("WHERE".to_string()),
            Token::Whitespace,
            Token::Identifier("name".to_string()),
            Token::Whitespace,
            Token::Keyword("LIKE".to_string()),
            Token::Whitespace,
            Token::Literal("'J%'".to_string()),
            Token::Punctuation(';'),
        ];

        assert_eq!(tokens, expected_tokens);
    }
}
