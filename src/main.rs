mod tokenizer;
mod parser;
mod lexer;

fn main() {
  let result = run().unwrap_or_else(|err| {
    eprintln!("[ERROR] {}", err);
    std::process::exit(1);
  });

  println!("[INFO] result: {}", result);
}


fn run() -> Result<f64,String> {
  let args: Vec<String> = std::env::args().collect();

  let arg = if let Some(s) = args.get(1) {
    s
  } else {
    return Err("where is string to parse".to_string());
  };

  let expr = parser::Parser::parse(&arg)?;

  Ok(expr.calc())
}
