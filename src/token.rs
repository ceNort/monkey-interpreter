#[derive(Debug)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers and literals
    IDENT, // add, foobar, x, y
    INT,  // 1234323

    // Operators
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET
}

impl TokenType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenType::ILLEGAL     => "ILLEGAL",
            TokenType::EOF         => "EOF",
            TokenType::IDENT       => "IDENT",
            TokenType::INT         => "INT",
            TokenType::ASSIGN      => "=",
            TokenType::PLUS        => "+",
            TokenType::COMMA       => ",",
            TokenType::SEMICOLON   => ";",
            TokenType::LPAREN      => "(",
            TokenType::RPAREN      => ")",
            TokenType::LBRACE      => "{",
            TokenType::RBRACE      => "}",
            TokenType::FUNCTION    => "FUNCTION",
            TokenType::LET         => "LET"
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub t_type:  TokenType,
    pub literal: String,
}

impl Token {
    pub fn lookup_ident(ident: &str) -> TokenType {
        match ident {
            "fn" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            _ => TokenType::IDENT,
        }
    }
}