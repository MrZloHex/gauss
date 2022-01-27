#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::types::*;

struct Location {
    filename: String,
    row: usize,
    column: usize
}

enum TokenType {

}

struct Token {
    location: Location,
    content: String,
    tok_type: TokenType
}

pub struct Lexer {
    code: String,
    location: Location,
    line: String
}

impl Lexer {
    pub fn new(code: String, filepath: String) -> Lexer {
        let (ln, _) = code.split_once('\n').unwrap();
        let line = ln.to_string();
        Lexer {
            code,
            location: Location {
                row: 0,
                column: 0,
                filename: filepath
            },
            line
        }
    }
    
    pub fn lex(&mut self) {
        
    }
    
    fn next_token(&mut self) -> Result<Token, bool> {
        self.line = self.line.trim_start().to_string();
        while self.line.len() == 0 && self.code.len() > 0 {
            self.next_line();
            self.line = self.line.trim_start().to_string();
        }

        if self.line.len() == 0 { return Err(false) }
        else {return Err(false)}
    }

    fn next_line(&mut self) {
        let (ln, cd) = self.code.split_once('\n').unwrap();
        self.line = ln.to_string();
        self.code = cd.to_string();

        self.location.row += 1;
    }
}

