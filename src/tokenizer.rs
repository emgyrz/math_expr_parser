use std::fmt;
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Op {
  Mul,
  Div,
  Sub,
  Add,
}


impl fmt::Display for Op {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(),fmt::Error> {
    let s = match self {
      Op::Mul => "*",
      Op::Div => "/",
      Op::Sub => "-",
      Op::Add => "+",
    };
    write!(f, "{}", s)
  }
}



#[derive(Debug,Clone,Copy)]
pub enum Token {
  Digit(f64),
  Bracket(bool),
  Op(Op),
}


fn digit_checker(ch: char) -> bool {
  ch.is_ascii_digit() || ch == '.'
}
fn space_checker(ch: char) -> bool {
  ch.is_whitespace()
}

#[derive(Debug)]
pub struct Tokenizer {
  list: Vec<Token>,
  pointer: usize,
}

impl Tokenizer {
  pub fn tokenize(s: &str) -> Result<Vec<Token>, String> {
    let mut t = Tokenizer {
      pointer: 0,
      list: Vec::new(),
    };
    let s_len = s.len();
    while t.pointer < s_len {
      t.read(s)?;
      // t.trim();
    }
    Ok(t.list)
  }

  fn push(&mut self, token: Token, count: usize) {
    self.list.push(token);
    self.pointer += count;
  }

  fn read(&mut self, src: &str) -> Result<(), String> {
    let s = &src[self.pointer..];
    let ch = s.chars().nth(0).unwrap();
    let next = s.chars().nth(1);
    match ch {
      ' ' => {
        self.pointer += 1;
      }
      '(' => self.push(Token::Bracket(true), 1),
      ')' => self.push(Token::Bracket(false), 1),
      '*' => self.push(Token::Op(Op::Mul), 1),
      '/' => self.push(Token::Op(Op::Div), 1),
      '+' => {
        if let Some(n) = next {
          if !space_checker(n) {
            self.pointer += 1;
            return Ok(());
          }
        }
        self.push(Token::Op(Op::Add), 1)
      },
      '-' => {
        self.handle_sub(next)?;
      },
      x if x.is_ascii_digit() => self.handle_digit(s)?,
      _ => return Err(format!("unrecognized input `{}` at point {}", ch, self.pointer + 1)),
    }

    Ok(())
  }


  fn handle_digit(&mut self, s_part: &str) -> Result<(), String> {
    let (digit_str, digit_str_len) = Tokenizer::take_while(s_part, digit_checker);
    if let Ok(d) = digit_str.parse() {
      self.push(Token::Digit(d), digit_str_len);
      Ok(())
    } else {
      Err(format!("cannot parse digit `{}` at point {}", digit_str, self.pointer + 1))
    }
  }


  fn handle_sub(&mut self, next_ch: Option<char> ) -> Result<(), String> {
    if let Some(n) = next_ch {
      if !space_checker(n) {
        if digit_checker(n) || n == '(' {
          self.push(Token::Digit(-1.0), 0);
          self.push(Token::Op(Op::Mul), 1);
          return Ok(());
        } else {
          return Err(format!("invalid operand sequence at point {}", self.pointer + 1));
        }
      }
    }
    self.push(Token::Op(Op::Sub), 1);
    Ok(())
  }


  fn take_while<F>(s: &str, checker: F) -> (&str, usize)
  where
    F: Fn(char) -> bool,
  {
    let len = s.len();
    let mut idx = 1;
    if idx < len {
      let mut ch = s.chars().nth(idx).unwrap();
      while checker(ch) {
        idx += 1;
        if idx == len {
          break;
        }
        ch = s.chars().nth(idx).unwrap();
      }
    }

    (&s[..idx], idx)
  }

  // fn trim(&mut self) {
  //   let r = self.list.iter().skip_while(|t| true);
  // }
}
