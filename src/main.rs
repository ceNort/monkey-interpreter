mod lexer;
mod token;

use token::*;
use lexer::*;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_next_token_for_token_types() {
        let input = r#"let five = 5;
	let ten = 10;

	let add = fn(x, y) {
	  x + y;
	};

	let result = add(five, ten);"#;

        let tests = vec![
            (TokenType::LET, "let"),
            (TokenType::IDENT, "five"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "ten"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "10"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "add"),
            (TokenType::ASSIGN, "="),
            (TokenType::FUNCTION, "fn"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "x"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "y"),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::IDENT, "x"),
            (TokenType::PLUS, "+"),
            (TokenType::IDENT, "y"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::LET, "let"),
            (TokenType::IDENT, "result"),
            (TokenType::ASSIGN, "="),
            (TokenType::IDENT, "add"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "five"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "ten"),
            (TokenType::RPAREN, ")"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::EOF, " "),
        ];

        // ORIG
        // let input = "=+(){},;";
        //
        // let tests = vec![
        //     (TokenType::ASSIGN,    "="),
        //     (TokenType::PLUS,      "+"),
        //     (TokenType::LPAREN,    "("),
        //     (TokenType::RPAREN,    ")"),
        //     (TokenType::LBRACE,    "{"),
        //     (TokenType::RBRACE,    "}"),
        //     (TokenType::COMMA,     ","),
        //     (TokenType::SEMICOLON, ";"),
        //     (TokenType::EOF,       ""),
        // ];

        let mut l = Lexer::new(input);

        for (index, (ttype, literal)) in tests.iter().enumerate() {
            let tok: Token = l.next_token();

            assert_eq!(tok.t_type.as_str(), ttype.as_str());
            assert_eq!(tok.literal, *literal.to_string());
        }
    }
}
