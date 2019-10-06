use crate::tokenizer::{Op, Token};
use std::collections::VecDeque;

fn op_priority(op: Op) -> u8 {
  match op {
    Op::Add | Op::Sub => 3,
    Op::Mul | Op::Div => 4,
    Op::Pow => 5,
  }
}

#[derive(Debug)]
pub enum LexerToken {
  Digit(f64),
  Op(Op),
}

#[derive(Default, Debug)]
pub struct Lexer {
  stack: VecDeque<Token>,
  output: Vec<LexerToken>,
}

impl Lexer {
  pub fn analyze(tokens: &mut VecDeque<Token>) -> Result<Vec<LexerToken>, String> {
    let mut lexer = Lexer::default();
    while let Some(t) = tokens.pop_front() {
      lexer.handle(t)?;
    }
    lexer.clear_stack();
    Ok(lexer.output)
  }

  fn handle(&mut self, token: Token) -> Result<(), String> {
    match token {
      Token::Digit(d) => {
        self.output.push(LexerToken::Digit(d));
      }
      Token::Bracket(is_opening) => {
        if is_opening {
          self.stack.push_back(token);
        } else {
          self.move_brackets_from_stack()?;
        }
      }
      Token::Op(op) => {
        self.push_operation(op);
      }
    }
    // println!("{} {:?}", self.stack);
    Ok(())
  }

  fn push_operation(&mut self, op: Op) {
    let pushing_priority = op_priority(op);

    while let Some(t) = self.stack.back() {
      if let Token::Op(op) = t {
        if op_priority(*op) >= pushing_priority {
          self.output.push(LexerToken::Op(*op));
          self.stack.pop_back();
        } else {
          break;
        }
      } else {
        break;
      }
    }

    self.stack.push_back(Token::Op(op));
  }

  fn clear_stack(&mut self) {
    while let Some(t) = self.stack.pop_back() {
      if let Token::Op(op) = t {
        self.output.push(LexerToken::Op(op));
      }
    }
  }

  fn move_brackets_from_stack(&mut self) -> Result<(), String> {
    let mut tmp: VecDeque<Op> = VecDeque::new();
    while let Some(t) = self.stack.pop_back() {
      if let Token::Bracket(is_opening) = t {
        if is_opening {
          for op in tmp.drain(..) {
            self.output.push(LexerToken::Op(op));
          }
          return Ok(());
        }
      }
      if let Token::Op(op) = t {
        tmp.push_back(op);
      }
    }

    Err("not found pair for bracket".to_string())
  }
}
