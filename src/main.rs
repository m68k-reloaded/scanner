#![feature(or_patterns)]

mod token;

use token::Range;
use token::Token;

fn main() {
    println!("Hello, world!");
    let errors = vec![];

    let tokens: Vec<Token> = scan("Hello, world!", &errors).collect();
}

pub fn scan<'a, 'b>(source: &'a str, errors: &'b Vec<String>) -> Scanner<'a, 'b> {
    Scanner {
        errors,
        offset: 0,
        rest: source,
        cursor: 0,
    }
}

struct Scanner<'a, 'b> {
    errors: &'b Vec<String>,
    rest: &'a str, // The rest of the original source code.
    offset: usize, // The offset to the start of the original source.
    cursor: usize, // The cursor relative to the offset.
}

impl<'a> Scanner<'a, '_> {
    fn is_at_end(self) -> bool {
        self.rest.len() == 0
    }

    fn flush(&self) {
        self.offset += self.cursor;
        self.rest = &self.rest[self.cursor..];
        self.cursor = 0;
    }

    fn lexeme(self) -> String {
        self.rest.chars().take(self.cursor).collect()
    }

    fn peek(self) -> char {
        match self.rest.chars().nth(0) {
            Some(character) => character,
            None => '\0',
        }
    }

    fn advance(&self) -> char {
        self.cursor += 1;
        self.peek()
    }

    fn advance_while<Test>(&self, test: Test) -> String
    where
        Test: Fn(char) -> bool,
    {
        while test(self.peek()) {
            self.advance();
        }
        self.lexeme()
    }

    fn range(self) -> Range {
        self.offset..self.offset + self.cursor
    }

    fn scan_next_token(&self) -> Result<Token, String> {
        self.flush();

        match (self.advance(), self.peek()) {
            ('(', _) => Ok(Token::OpeningParen(self.range())),
            (')', _) => Ok(Token::ClosingParen(self.range())),
            (',', _) => Ok(Token::Comma(self.range())),
            ('.', _) => Ok(Token::Dot(self.range())),
            ('+', _) => Ok(Token::Plus(self.range())),
            ('#', _) => Ok(Token::NumberSign(self.range())),
            (':', _) => Ok(Token::Colon(self.range())),
            ('0'..='9', _) | ('-', '0'..='9') => self.parse_decimal_number(),
            ('$', _) => self.parse_hex_number(),
            ('-', _) => Ok(Token::Minus(self.range())),
            ('*', _) => self.parse_comment(),
            (' ' | '\t' | ' ', _) => Ok(Token::Whitespace(self.range())),
            ('\r', '\n') => {
                self.advance();
                Ok(Token::Newline(self.range()))
            }
            ('\n', _) => Ok(Token::Newline(self.range())),
            ('a'..='z' | 'A'..='Z' | '0'..='9' | '_', _) => self.parse_identifier(),
            _ => Err(String::from("No match.")),
        }
    }

    fn parse_decimal_number(&self) -> Result<Token, String> {
        let number = self.advance_while(|c| ('0'..'9').contains(&c));
        let number: u32 = match number.parse() {
            Ok(number) => number,
            Err(err) => return Err(String::from("Cannot parse decimal number.")),
        };
        Ok(Token::Number(self.range(), number))
    }

    fn parse_hex_number(&self) -> Result<Token, String> {
        let number = self.advance_while(|c| ('0'..'9').contains(&c) || ('a'..'f').contains(&c));
        let number: u32 = match u32::from_str_radix(&number, 16) {
            Ok(number) => number,
            Err(err) => return Err(String::from("Cannot parse hex number.")),
        };
        Ok(Token::Number(self.range(), number))
    }

    fn parse_comment(&self) -> Result<Token, String> {
        let content = self.advance_while(|c| c != '\n');
        Ok(Token::Comment(self.range(), String::from(content)))
    }

    fn parse_identifier(&self) -> Result<Token, String> {
        let identifier = self.advance_while(|c| {
            ('a'..='z').contains(&c)
                || ('A'..='Z').contains(&c)
                || ('0'..='9').contains(&c)
                || c == '_'
        });
        Ok(Token::Identifier(self.range(), String::from(identifier)))
    }
}

impl Iterator for Scanner<'_, '_> {
    type Item = Token;

    fn next(&mut self) -> std::option::Option<Token> {
        while !self.is_at_end() {
            match self.scan_next_token() {
                Ok(token) => return Some(token),
                Err(error) => self.errors.push(error),
            }
        }
        None
    }
}
