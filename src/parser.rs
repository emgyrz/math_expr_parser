use crate::tokenizer::{Op,Tokenizer};
use crate::lexer::{LexerToken,Lexer};
use std::collections::VecDeque;


#[derive(Debug)]
pub enum Expr {
  Inner(Box<Expr>, Op, Box<Expr>),
  Digit(f64),
}
impl Expr {
  pub fn calc(&self) -> f64 {
    match self {
      Expr::Digit(d) => *d,
      Expr::Inner(x, op, y) => match op {
        Op::Mul => x.calc() * y.calc(),
        Op::Div => x.calc() / y.calc(),
        Op::Sub => x.calc() - y.calc(),
        Op::Add => x.calc() + y.calc(),
        Op::Pow => x.calc().powf( y.calc() ),
      },
    }
  }
}


#[derive(Debug, Default)]
pub struct Parser {
  stack: VecDeque<Expr>,
  pointer: usize,
}

impl Parser {
  pub fn parse(s: &str) -> Result<Expr,String> {

    let tokens = Tokenizer::tokenize(s)?;
    let posfix_tokens =  Lexer::analyze(tokens)?;

    let mut p = Parser::default();
    let len = posfix_tokens.len();
    while p.pointer < len {
      p.read(&posfix_tokens[p.pointer])?;
    }


    if p.stack.len() != 1 {
      return Err("error parsing expression".to_string());
    }

    Ok(p.stack.pop_front().unwrap())
  }

  fn read(&mut self, token: &LexerToken) -> Result<(),String> {
    match token {
      LexerToken::Digit(d) => {
        self.stack.push_back(Expr::Digit(*d))
      },
      LexerToken::Op(op) => {
        let len = self.stack.len();
        if len < 2 {
          return Err(format!("unexpected operation {}", op));
        }
        let exp2 = self.stack.pop_back().unwrap();
        let exp1 = self.stack.pop_back().unwrap();
        self.stack.push_back(Expr::Inner(Box::new(exp1), *op, Box::new(exp2)));
      }
    }

    self.pointer += 1;

    Ok(())
  }

}
