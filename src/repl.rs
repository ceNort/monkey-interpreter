use super::token::*;
use super::lexer::*;
use std::io::{self, Write};
use std::process::exit;

pub fn start() {
    let mut input = String::new();

    loop { // Loop until "exit"
        // Print prompt
        print!(">> ");
        io::stdout().flush().unwrap();

        // Read input
        io::stdin().read_line(&mut input).unwrap();

        // Check for exit
        if input.trim() == String::from("exit") {
            exit(1);
        }

        // Create lexer object for input
        let mut lexer = Lexer::new(&input);

        // Iterate through tokens and display
        let mut token = lexer.next_token();
        while token.t_type != TokenType::EOF {
            println!("{:?}", token);
            token = lexer.next_token();
        }

        // Clear input
        input.clear();
    }
}