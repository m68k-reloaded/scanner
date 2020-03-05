pub use m68k_reloaded_common::Range;

#[derive(Debug)]
pub enum Token {
  // Single characters.
  OpeningParen(Range), // (
  ClosingParen(Range), // )
  Comma(Range),        // ,
  Dot(Range),          // .
  Minus(Range),        // -
  Plus(Range),         // +
  NumberSign(Range),   // #
  Colon(Range),        // :

  // Literals.
  Comment(Range, String),
  Identifier(Range, String),
  Number(Range, u32),

  // Whitespace.
  Whitespace(Range),
  Newline(Range),
}

/*class Token {
  String toString() {
    return '${type.toString().substring('TokenType.'.length)} at $location: "$lexeme" (Literal: $literal)';
  }
}*/
