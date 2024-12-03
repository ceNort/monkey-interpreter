#[allow(non_camel_case_types)]

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
    MINUS,
    BANG,
    SLASH,
    ASTERISK,

    GT,
    LT,

    EQ,
    NOT_EQ,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
    IF,
    ELSE,
    TRUE,
    FALSE,
    RETURN,
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
            TokenType::MINUS       => "-",
            TokenType::BANG        => "!",
            TokenType::SLASH       => "/",
            TokenType::ASTERISK    => "*",
            TokenType::GT          => ">",
            TokenType::LT          => "<",
            TokenType::EQ          => "==",
            TokenType::NOT_EQ      => "!=",
            TokenType::COMMA       => ",",
            TokenType::SEMICOLON   => ";",
            TokenType::LPAREN      => "(",
            TokenType::RPAREN      => ")",
            TokenType::LBRACE      => "{",
            TokenType::RBRACE      => "}",
            TokenType::FUNCTION    => "FUNCTION",
            TokenType::LET         => "LET",
            TokenType::IF          => "IF",
            TokenType::ELSE        => "ELSE",
            TokenType::TRUE        => "TRUE",
            TokenType::FALSE       => "FALSE",
            TokenType::RETURN      => "RETURN",
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
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            "return" => TokenType::RETURN,
            _ => TokenType::IDENT,
        }
    }
}