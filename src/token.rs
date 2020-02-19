enum Token {
    OpeningParen(Location),
    ClosingParen(Location),
    Comma(Location),
    Dot(Location),
    Minus(Location),
    Plus(Location),
    NumberSign(Location), // #
    Colon(Location),      // :

    // Literals.
    Comment(Location, String),
    Identifier(Location, String),
    Number(Location, u32),
}

/*class Token {
  String toString() {
    return '${type.toString().substring('TokenType.'.length)} at $location: "$lexeme" (Literal: $literal)';
  }
}*/
