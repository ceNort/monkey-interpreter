use super::token::*;
pub struct Lexer {
    pub input: String,
    pub position: usize,  // Current position in input (points to current char)
    pub read_position: usize, // Current reading position in input (after current char)
    pub ch: Option<char>, // Current char under examination
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: None
        };
        l.read_char();
        return l;
    }

    pub fn read_char(&mut self) {
        self.ch = self.input.chars().nth(self.read_position);
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok: Token = match self.ch {
            Some('=') => new_token(TokenType::ASSIGN, self.ch.unwrap()),
            Some(';') => new_token(TokenType::SEMICOLON, self.ch.unwrap()),
            Some('(') => new_token(TokenType::LPAREN, self.ch.unwrap()),
            Some(')') => new_token(TokenType::RPAREN, self.ch.unwrap()),
            Some(',') => new_token(TokenType::COMMA, self.ch.unwrap()),
            Some('+') => new_token(TokenType::PLUS, self.ch.unwrap()),
            Some('{') => new_token(TokenType::LBRACE, self.ch.unwrap()),
            Some('}') => new_token(TokenType::RBRACE, self.ch.unwrap()),
            None => new_token(TokenType::EOF, ' '),
            _ => {
                if is_letter(self.ch.unwrap()) {
                    let literal = self.read_identifier().to_string();
                    return Token {
                        t_type: Token::lookup_ident(&literal),
                        literal,
                    }
                } else if is_digit(self.ch.unwrap()) {
                    return Token {
                        t_type: TokenType::INT,
                        literal: self.read_number().to_string(),
                    }
                } else {
                    new_token(TokenType::ILLEGAL, self.ch.unwrap())
                }
            }
        };

        self.read_char();
        return tok;
    }

    pub fn read_identifier(&mut self) -> &str {
        let start_position = self.position;
        while is_letter(self.ch.unwrap()) {
            self.read_char()
        }
        // Return slice of self.input from start_position to current position
        return &self.input[start_position..self.position]
    }

    pub fn read_number(&mut self) -> &str {
        let start_position = self.position;
        while is_digit(self.ch.unwrap()) {
            self.read_char()
        }
        // Return slice of self.input from start_position to current position
        return &self.input[start_position..self.position]
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if matches!(ch, ' ' | '\t' | '\n' | '\r') {
                self.read_char();
            } else {
                break;
            }
        }
    }
}

pub fn is_letter(ch: char) -> bool {
    return ch.is_alphabetic() || ch == '_'
}

pub fn is_digit(ch: char) -> bool {
    return ch.is_numeric()
}

pub fn new_token(ttype: TokenType, ch: char) -> Token {
    Token {
        t_type: ttype,
        literal: String::from(ch)
    }
}
