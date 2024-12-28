use rgparser::tokenizer::{Token, Tokenizer};

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

        let expected_tokens = vec![
            Token::Keyword("CREATE".to_string()),
            Token::Whitespace,
            Token::Entity("TABLE".to_string()),
            Token::Whitespace,
            Token::Identifier("users".to_string()),
            Token::Whitespace,
            Token::Punctuation('('),
            Token::Identifier("id".to_string()),
            Token::Whitespace,
            Token::Datatype("INT".to_string()),
            Token::Punctuation(','),
            Token::Whitespace,
            Token::Identifier("name".to_string()),
            Token::Whitespace,
            Token::Datatype("VARCHAR".to_string()),
            Token::Punctuation('('),
            Token::Literal("255".to_string()),
            Token::Punctuation(')'),
            Token::Punctuation(')'),
            Token::Punctuation(';'),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_insert_into() {
        let sql_query = "INSERT INTO users (name, age) VALUES ('John', 30);";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![
            Token::Keyword("INSERT".to_string()),
            Token::Whitespace,
            Token::Keyword("INTO".to_string()),
            Token::Whitespace,
            Token::Identifier("users".to_string()),
            Token::Whitespace,
            Token::Punctuation('('),
            Token::Identifier("name".to_string()),
            Token::Punctuation(','),
            Token::Whitespace,
            Token::Identifier("age".to_string()),
            Token::Punctuation(')'),
            Token::Whitespace,
            Token::Keyword("VALUES".to_string()),
            Token::Whitespace,
            Token::Punctuation('('),
            Token::Literal("'John'".to_string()),
            Token::Punctuation(','),
            Token::Whitespace,
            Token::Literal("30".to_string()),
            Token::Punctuation(')'),
            Token::Punctuation(';'),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_update() {
        let sql_query = "UPDATE users SET age = 31 WHERE name = 'John';";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![
            Token::Keyword("UPDATE".to_string()),
            Token::Whitespace,
            Token::Identifier("users".to_string()),
            Token::Whitespace,
            Token::Keyword("SET".to_string()),
            Token::Whitespace,
            Token::Identifier("age".to_string()),
            Token::Whitespace,
            Token::Operator('='),
            Token::Whitespace,
            Token::Literal("31".to_string()),
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

    #[test]
    fn test_delete() {
        let sql_query = "DELETE FROM users WHERE name = 'John';";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![
            Token::Keyword("DELETE".to_string()),
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

    #[test]
    fn test_truncate() {
        let sql_query = "TRUNCATE TABLE users;";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![
            Token::Keyword("TRUNCATE".to_string()),
            Token::Whitespace,
            Token::Entity("TABLE".to_string()),
            Token::Whitespace,
            Token::Identifier("users".to_string()),
            Token::Punctuation(';'),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_drop_table() {
        let sql_query = "DROP TABLE users;";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![
            Token::Keyword("DROP".to_string()),
            Token::Whitespace,
            Token::Entity("TABLE".to_string()),
            Token::Whitespace,
            Token::Identifier("users".to_string()),
            Token::Punctuation(';'),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_group_by() {
        let sql_query = "SELECT name, COUNT(*) FROM users GROUP BY name;";
        let tokenizer = Tokenizer::new(sql_query);
        let tokens = tokenizer.tokenize();

        let expected_tokens = vec![
            Token::Keyword("SELECT".to_string()),
            Token::Whitespace,
            Token::Identifier("name".to_string()),
            Token::Punctuation(','),
            Token::Whitespace,
            Token::Function("COUNT".to_string()),
            Token::Punctuation('('),
            Token::Operator('*'),
            Token::Punctuation(')'),
            Token::Whitespace,
            Token::Keyword("FROM".to_string()),
            Token::Whitespace,
            Token::Identifier("users".to_string()),
            Token::Whitespace,
            Token::Keyword("GROUP".to_string()),
            Token::Whitespace,
            Token::Keyword("BY".to_string()),
            Token::Whitespace,
            Token::Identifier("name".to_string()),
            Token::Punctuation(';'),
        ];

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
