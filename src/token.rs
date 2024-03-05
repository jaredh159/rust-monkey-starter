#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
  Assign,
  Plus,
  Eof,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
  pub token_type: TokenType,
  pub literal: String,
}

impl Token {
  pub const fn new(token_type: TokenType, literal: String) -> Self {
    Self {
      token_type,
      literal,
    }
  }
}
