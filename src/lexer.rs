use crate::token::{Token, TokenType};

pub struct Lexer {
  input: Vec<char>,
  position: usize,
  read_position: usize,
  ch: char,
}

impl Lexer {
  pub fn new(input: String) -> Self {
    let mut lexer = Self {
      input: input.chars().collect(),
      position: 0,
      read_position: 0,
      ch: '\0',
    };
    lexer.read_char();
    lexer
  }

  // NB: more idiomatic rust would be to implement `Iterator` for Lexer
  // but this will keep things closer to Thorsten's original code
  pub fn next_token(&mut self) -> Token {
    let token = match self.ch {
      '=' => Token::new(TokenType::Assign, self.ch.to_string()),
      _ => Token::new(TokenType::Eof, self.ch.to_string()),
    };
    self.read_char();
    token
  }

  fn read_char(&mut self) {
    if self.read_position >= self.input.len() {
      self.ch = '\0';
    } else {
      self.ch = self.input[self.read_position];
    }
    self.position = self.read_position;
    self.read_position += 1;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_next_token() {
    let cases = vec![
      (TokenType::Assign, "="),
      // NB: the rest of his first batch of tests below
      // not all of the enum variants are defined yet!

      // (TokenType::Plus, "+"),
      // (TokenType::LParen, "("),
      // (TokenType::RParen, ")"),
      // (TokenType::LBrace, "{"),
      // (TokenType::RBrace, "}"),
      // (TokenType::Comma, ","),
      // (TokenType::Semicolon, ";"),
      // (TokenType::Eof, ""),
    ];

    for (expected_type, expected_literal) in cases {
      let input = expected_literal.to_string();
      let mut lexer = Lexer::new(input);
      let token = lexer.next_token();
      assert_eq!(token.token_type, expected_type);
      assert_eq!(token.literal, expected_literal);
    }
  }
}
