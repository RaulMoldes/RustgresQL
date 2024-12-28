// statement_builder.rs
// Builds sql statements from lists of tokens.
use crate::tokenizer::Token;


#[derive(Debug, PartialEq, Clone)]
enum SqlClauses {
    Create {
        item: Token,
        name: Token,
        content: Vec<Token>,
    },
    Select {
        columns: Vec<Token>,
        from_clause: Box<SqlClauses>,
        where_clause: Option<Box<SqlClauses>>,
    },
    Insert {
        table: Token,
        columns: Vec<Token>,
        values: Box<SqlClauses>,
    },
    Update {
        table: Token,
        set_clause:  Box<SqlClauses>,
        where_clause: Option<Box<SqlClauses>>,
    },
    Drop {
        item: Token,
        name: Token,
    },
    Delete {
        from_clause: Box<SqlClauses>,
        where_clause: Option<Box<SqlClauses>>,
    },
    Alter {
        item: Token,
        name: Token,
        action: Box<SqlClauses>,
    },
    From {
        table: Token,
        join_clause: Option<Box<SqlClauses>>,
    },
    Join {
        table: Token,
        on_clause: Box<SqlClauses>,
    },
    Set {
        column: Token,
        value: Token,
    },
    Where {
        column: Token,
        operator: Token,
        value: Token,
    },
    Rollback,
    Commit,
    Begin,
    End,
    Unknown,

}



fn build_clause(tokens: Vec<Token>)-> SqlClauses {
    let mut tokens = tokens.into_iter().filter(|token| !matches!(token, Token::Whitespace));
    let first_token = tokens.next().unwrap();
    match first_token {
        Token::Keyword(k) => match k.as_str() {
            "CREATE" => {
                let item = match tokens.next().unwrap() {
                    Token::Entity(r#type) => Token::Entity(r#type),
                    _ => panic!("Expected an entity, found: {:?}", tokens.next().unwrap()),
                };
                let name = match tokens.next().unwrap() {
                    Token::Identifier(r#type) => Token::Identifier(r#type),
                    _ => panic!("Expected an identifier, found: {:?}", tokens.next().unwrap()),
                };
                let content = tokens.collect();
                SqlClauses::Create {
                    item,
                    name,
                    content,
                }
            },
        _ => panic!("Unsupported keyword: {:?}", k),
    },
    _ => panic!("First token must be a keyword, found: {:?}", first_token),
}
}


#[cfg(test)]

mod tests {
    use super::*;
    use crate::tokenizer::Token;

    #[test]
    fn test_build_clause() {
        let tokens = vec![
            Token::Keyword("CREATE".to_string()),
            Token::Entity("TABLE".to_string()),
            Token::Identifier("users".to_string()),
            Token::Punctuation('('),
            Token::Identifier("id".to_string()),
            Token::Datatype("INT".to_string()),
            Token::Punctuation(','),
            Token::Identifier("name".to_string()),
            Token::Datatype("TEXT".to_string()),
            Token::Punctuation(')'),
        ];
        let expected = SqlClauses::Create {
            item: Token::Entity("TABLE".to_string()),
            name: Token::Identifier("users".to_string()),
            content: vec![
                Token::Punctuation('('),
                Token::Identifier("id".to_string()),
                Token::Datatype("INT".to_string()),
                Token::Punctuation(','),
                Token::Identifier("name".to_string()),
                Token::Datatype("TEXT".to_string()),
                Token::Punctuation(')'),
            ],
        };
        assert_eq!(build_clause(tokens), expected);
    }
}