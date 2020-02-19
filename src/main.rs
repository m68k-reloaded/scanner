fn main() {
    println!("Hello, world!");
}

struct ScannerState {
    source: String,
    start: u8,
    line: u8,
    column: u8,
    current: u8,
    tokens: Vector<Token>,
}

impl ScannerState {
    fn new(source: String) -> ScannerState {
        ScannerState {
            source,
            start: 0,
            line: 1,
            column: 1,
            current: 0,
            tokens: Vector<Token>(),
        }
    }

    fn locationWithZeroLength(&self) -> Location {
        Location {
            line,
            column,
            length: 0,
        }
    }

    fn isAtEnd() -> bool {
        current >= source.length
    }

    fn peek() -> bool {
        if isAtEnd() {
            '\0'
        } else {
            source[current]
        }
    }

    fn currentLexeme() -> String {
        source[start .. current]
    }

    fn advance() -> char {
        current += 1;
        source[current]
    }

    fn advanceWhile() {
        /*String advanceWhile(bool Function(String char) predicate) {
            while (predicate(peek()) && !isAtEnd) {
            advance();
            }
            return source.substring(start, current);
        }*/
    }

    fn addToken(token: Token) {
        /*tokens.add(Token(
            type: type,
            location: location.copyWith(length: currentLexeme.length),
            lexeme: currentLexeme,
        ));*/
        tokens.add(token);
        column += current - start;
        start = current;
    }
}

fn scan(source: String, errorCollector: ErrorCollector) -> Vector<Token> {
    let state = ScannerState::new(source);

    while !state.isAtEnd() {
        scanNextToken(state, errorCollector);
    }

    state.tokens
}

fn scanNextToken(state: ScannerState, errorCollector: ErrorCollector) {
    let location = state.locationWithZeroLength();

    match (state.advance(), state.peek()) {
        ('(', _) => state.addToken(Token::OpeningParen(location)),
        (')', _) => state.addToken(Token::ClosingParen(location)),
        (',', _) => state.addToken(Token::Comma(location)),
        ('.', _) => state.addToken(Token::Dot(location)),
        ('+', _) => state.addToken(Token::Plus(location)),
        ('#', _) => state.addToken(Token::NumberSign(location)),
        (':', _) => state.addToken(Token::Colon(location)),
        ('0'..='9', _) | ('-', '0'..='9') => parseDecimalNumber(state),
        ('$', _) => parseHexNumber(state),
        ('-', _) => state.addToken(Token::Minus(location)),
        ('*', _) => parseComment(state),
        (' ' | '\t', _) => {
            state.addToken(Tokens::Whitespace(location));
            state.col += 1;
            state.start = start.current;
        },
        ('\r' | '\n', _) => {
            if state.currentToken() == '\r' {
                
            }
            state.addToken(Token::Newline(location));
            state.line += 1;
            state.start = state.current;
            state.col = 1;
        },
        ('a'..'z' | 'A'..'Z' | '0'..'9' | '_', _) => parseIdentifier(state),
        _ => errorCollector.addError(...),
    }
}

void _parseToken<T>({
  @required _ScannerState state,
  @required TokenType type,
  @required bool Function(String char) selector,
}) {
  assert(state != null);
  state.advanceWhile(selector);
  state.addToken(type);
}

void _parseDecimalNumber(_ScannerState state) => _parseToken(
      state: state,
      type: TokenType.number,
      selector: _isDecimalDigit,
    );
void _parseHexNumber(_ScannerState state) {
  assert(state != null);

  if (state.peek() == '-') state.advance();
  _parseToken(
    state: state,
    type: TokenType.number,
    selector: _isHexDigit,
  );
}

void _parseComment(_ScannerState state) => _parseToken(
      state: state,
      type: TokenType.comment,
      selector: (c) => !_newline.contains(c),
    );
void _parseIdentifier(_ScannerState state) => _parseToken(
      state: state,
      type: TokenType.identifier,
      selector: _isLetterDigitUnderscore,
    );
