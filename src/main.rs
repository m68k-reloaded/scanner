mod token;

use token::Range;
use token::Token;

fn main() {
    println!("Hello, world!");
}

fn scan(source: &str, errors: Vec<String>) -> Vec<Token> {
    let tokens = Vec::new();
    let state = ScannerState::new(source);

    while !state.is_at_end() {
        match scan_next_token(state) {
            Ok(token) => tokens.push(token),
            Err(error) => errors.push(error),
        }
    }

    tokens
}

struct ScannerState {
    source: String,

    start: u16,
    current: u16,
}

impl ScannerState {
    fn new(source: &str) -> ScannerState {
        ScannerState {
            source: String::from(source),
            start: 1,
            current: 1,
        }
    }

    fn is_at_end(self) -> bool {
        usize::from(self.current) >= self.source.len()
    }

    fn start_new_token_parsing(&self) {
        self.start = self.current;
    }

    fn lexeme(self) -> String {
        String::from(&(self.source)[self.start..self.current])
    }

    fn range(self) -> Range {
        Range(self.start, self.current)
    }

    fn peek(self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.bytes()[self.current]
        }
    }

    fn advance(&self) -> char {
        self.current += 1;
        self.source[self.current]
    }

    fn advance_while<Test>(&self, test: Test) -> String
    where
        Test: Fn(char) -> bool,
    {
        while test(self.peek()) {
            self.advance();
        }
        self.source[self.start..self.current]
    }
}

fn scan_next_token(state: ScannerState) -> Result<Token, String> {
    state.start_new_token_parsing();
    let range = state.range();

    match (state.advance(), state.peek()) {
        ('(', _) => Ok(Token::OpeningParen(range)),
        (')', _) => Ok(Token::ClosingParen(range)),
        (',', _) => Ok(Token::Comma(range)),
        ('.', _) => Ok(Token::Dot(range)),
        ('+', _) => Ok(Token::Plus(range)),
        ('#', _) => Ok(Token::NumberSign(range)),
        (':', _) => Ok(Token::Colon(range)),
        ('0'..='9', _) | ('-', '0'..='9') => parse_decimal_number(state),
        ('$', _) => parse_hex_number(state),
        ('-', _) => Ok(Token::Minus(range)),
        ('*', _) => parse_comment(state),
        (' ' | '\t' | ' ', _) => Ok(Token::Whitespace(range)),
        ('\n', _) => Ok(Token::Newline(range)),
        ('a'..='z' | 'A'..='Z' | '0'..='9' | '_', _) => parse_identifier(state),
        _ => Err(String::from("No match.")),
    }
}

fn parse_decimal_number(state: ScannerState) -> Result<Token, String> {
    let number = state.advance_while(|c| ('0'..'9').contains(&c));
    let number: u32 = match number.parse() {
        Ok(number) => number,
        Err(err) => return Err(String::from("Cannot parse decimal number.")),
    };
    Ok(Token::Number(state.range(), number))
}

fn parse_hex_number(state: ScannerState) -> Result<Token, String> {
    let number = state.advance_while(|c| ('0'..'9').contains(&c) || ('a'..'f').contains(&c));
    let number: u32 = match u32::from_str_radix(&number, 16) {
        Ok(number) => number,
        Err(err) => return Err(String::from("Cannot parse hex number.")),
    };
    Ok(Token::Number(state.range(), number))
}

fn parse_comment(state: ScannerState) -> Result<Token, String> {
    let content = state.advance_while(|c| c != '\n');
    Ok(Token::Comment(state.range(), content))
}

fn parse_identifier(state: ScannerState) -> Result<Token, String> {
    let identifier = state.advance_while(|c| {
        ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || ('0'..='9').contains(&c) || c == '_'
    });
    Ok(Token::Identifier(state.range(), identifier))
}
