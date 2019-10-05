use crate::tokenizer::{Op, Token};

fn op_priority(op: Op) -> u8 {
  match op {
    Op::Add | Op::Sub => 3,
    _ => 4,
  }
}

#[derive(Debug)]
pub enum LexerToken {
  Digit(f64),
  Op(Op),
}



#[derive(Default, Debug)]
pub struct Lexer {
  pointer: usize,
  stack: Vec<Token>,
  output: Vec<LexerToken>,
}

impl Lexer {
  pub fn analyze(tokens: &[Token]) -> Result<Vec<LexerToken>, String> {
    let mut lexer = Lexer::default();
    let len = tokens.len();
    while lexer.pointer < len {
      lexer.handle(&tokens[lexer.pointer])?;
    }
    lexer.clear_stack();
    Ok(lexer.output)
  }

  fn handle(&mut self, token: &Token) -> Result<(), String> {
    match token {
      Token::Digit(d) => {
        self.output.push(LexerToken::Digit( *d ));
      }
      Token::Bracket(is_opening) => {
        if *is_opening {
          self.stack.push(*token);
        } else {
          self.move_brackets_from_stack()?;
        }
      }
      Token::Op(op) => {
        self.push_operand(*op);
      }
    }
    self.pointer += 1;
    // println!("{} {:?}", self.pointer, self.stack);
    Ok(())
  }

  fn push_operand(&mut self, op: Op) {
    // let mut operand_ind_to_move = Vec::new();
    let pushing_priority = op_priority(op);
    // for (ind, t) in self.stack.iter().enumerate() {
    //   if let Token::Op(op) = t {
    //     if op_priority(*op) >= pushing_priority {
    //       operand_ind_to_move.push(ind);
    //     }
    //   }
    // }
    // operand_ind_to_move.iter().for_each(|ind| {
    //   self.output.push( self.stack.remove(*ind) );
    // });

    let mut stack_pointer = self.stack.len();
    while stack_pointer != 0 {
      if let Token::Op(op) = &self.stack[stack_pointer - 1] {
        if op_priority(*op) >= pushing_priority {
          self.output.push(LexerToken::Op(*op));
          self.stack.remove(stack_pointer - 1);
          stack_pointer -= 1;
        } else {
          break;
        }
      } else {
        break;
      }
    }

    self.stack.push(Token::Op(op));
  }

  fn clear_stack(&mut self) {
    for t in self.stack.drain(..).rev() {
      if let Token::Op(op) = t {
        self.output.push(LexerToken::Op(op));
      }
    }
  }

  fn move_brackets_from_stack(&mut self) -> Result<(), String> {
    // let part = &self.stack[..self.pointer];
    let last_opening = self.stack.iter().enumerate().rev().find(|(_, t)| match t {
      Token::Bracket(is_opening) => *is_opening,
      _ => false,
    });

    let ind = if let Some((ind, _)) = last_opening {
      ind
    } else {
      return Err(format!(
        "not found pair for bracket at position {}",
        self.pointer
      ));
    };
    for (ind, t) in self.stack.drain(ind..).enumerate().rev() {
      if ind != 0 {
        if let Token::Op(op) = t {
          self.output.push(LexerToken::Op(op));
        }
      }
    }

    Ok(())
  }
}
