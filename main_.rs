use std::{env, fs};

mod interpreter;    

#[derive(Debug, Clone)]
enum Token {
    NotToken, //yes
	Func, 
	Return,
	Int,
	Print,
	Read,
	While,
	If,
	Else,
	Break,
	Continue,
	LeftParen,
	RightParen,
	LeftCurly,
	RightCurly,
	LeftBracket,
	RightBracket,
	Comma,
	Semicolon,
	Plus, // yes 
	Subtract, // yes
	Multiply, // yes
	Divide, // yes
	Modulus, // yes
	Assign, // me
	Less, // me
	LessEqual, // me
	Greater, // me
	GreaterEqual, //me
	Equality, //me 
	NotEqual, // me
	Ident(String), // yes
	Num(i32), //yes
}

fn main() {
    // get commandline arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide an input file.");
        return;
    }

    if args.len() > 2 {
        println!("Too many commandline arguments.");
        return;
    }

    // read the entire file.
    let filename = &args[1];
    let result = fs::read_to_string(filename);
    let code = match result {
        Err(error) => {
            println!("**Error. File \"{}\": {}", filename, error);
            return;
        }

        Ok(code) => {
            code
        }
    };

    // Start Here!!
    let tokens = match lex(&code) {
        Err(error_message) => {
            println!("**Error**");
            println!("----------------------");
            println!("{}", error_message);
            println!("----------------------");
            return;
    }
    
    Ok(data) => data,
        
    };
    
    // print out the lexer tokens parsed.
    
    println!("----------------------");
    println!("Finished Lexing the file {}", filename);
    println!("Expression:");
    println!("{code}");
    println!("Here are the Results:");
    println!("----------------------");
    for t in &tokens {
        println!("{:?}", t);
    }

    // in : vector of tokens from func lex()
    // out : the production rules of the parser
    let mut index: usize = 0;
    match parse_program(&tokens, &mut index) {

      Ok(generated_code) => {
          println!("Intermediate Code:");
          println!("---------------------------");
          println!("{generated_code}");
          println!("---------------------------");
          interpreter::execute_ir(&generated_code);
      }

      Err(message) => {
          println!("**Error**");
          println!("----------------------");
          if tokens.len() == 0 {
              println!("No code has been provided.");
          } else {
              println!("Error: {message}");
              println!("----------------------");
          }
      }
    }
}

fn lex(mut code: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = vec![];
    while code.len() > 0 {
      let (success, token, rest) = lex_number(code);
      if success {
        code = rest; 
        tokens.push(token);
        continue;
      } 
   
      let (success, rest) = lex_space(code);
      if success {
        code = rest;
        continue;
      }

      if code.starts_with("#") {
        while code.len() > 0 {
          code = &code[1..];
          if code.starts_with("\n") {
            break;
          }
        }
        continue;
      }
  
      if code.starts_with("+") {
        code = &code[1..];
        tokens.push(Token::Plus);
        continue;
      }

      //TODO LEFT_PAREN\
      if code.starts_with("(") {
        code = &code[1..];
        tokens.push(Token::LeftParen);
        continue;
      }

      //TODO RIGHT_PAREN
      if code.starts_with(")") {
        code = &code[1..];
        tokens.push(Token::RightParen);
        continue;
      }

      //TODO LEFT_CURLY
      if code.starts_with("{") {
        code = &code[1..];
        tokens.push(Token::LeftCurly);
        continue;
      }

      //TODO RIGHT_CURLY
      if code.starts_with("}") {
        code = &code[1..];
        tokens.push(Token::RightCurly);
        continue;
      }

      //TODO LEFT_BRACKET
      if code.starts_with("[") {
        code = &code[1..];
        tokens.push(Token::LeftBracket);
        continue; 
      }

      //TODO RIGHT_BRACKET
      if code.starts_with("]") {
        code = &code[1..];
        tokens.push(Token::RightBracket);
        continue; 
      }

      //TODO COMMA
      if code.starts_with(",") {
        code = &code[1..];
        tokens.push(Token::Comma);
        continue;
      }
  
      if code.starts_with("-") {
        code = &code[1..];
        tokens.push(Token::Subtract);
        continue;
      }

      if code.starts_with(";") {
        code = &code[1..];
        tokens.push(Token::Semicolon);
        continue;
      }
  
      if code.starts_with("*") {
        code = &code[1..];
        tokens.push(Token::Multiply);
        continue;
      }
  
      if code.starts_with("/") {
        code = &code[1..];
        tokens.push(Token::Divide);
        continue;
      }
  
      if code.starts_with("%") {
        code = &code[1..];
        tokens.push(Token::Modulus);
        continue;
      }
      //inserted code here
      if code.starts_with("=") {
        if code.len() > 1 && &code[1..2] == "=" {
          code = &code[2..];
          tokens.push(Token::Equality);
        }
        else {
          code = &code[1..];
          tokens.push(Token::Assign);
        }
        continue;
      }

      //TODO KEYWORK IDENTIFIERS
      let (success, token, rest) = lex_keyword(code);
      if success {
        code = rest;
        tokens.push(token);
        continue;
      }
  

      if code.starts_with("<") {
        if code.len() > 1 && &code[1..2] == "=" {
          code = &code[2..];
          tokens.push(Token::LessEqual);
        }
        else {
          code = &code[1..];
          tokens.push(Token::Less);
        }
        continue;
      }

      if code.starts_with(">") {
        if code.len() > 1 && &code[1..2] == "=" {
          code = &code[2..];
          tokens.push(Token::GreaterEqual);
        }
        else {
          code = &code[1..];
          tokens.push(Token::Greater);
        }
        continue;
      }

      if code.starts_with("!") {
        if code.len() > 1 && &code[1..2] == "=" {
          code = &code[2..];
          tokens.push(Token::NotEqual);
        }
        else {
          code = &code[1..];
          tokens.push(Token::NotToken);
        }
        continue;
      }
      

      let (success, token, rest) = lex_identifier(code);
      if success {
        code = rest;
        tokens.push(token);
        continue;
      }
  
      let symbol = unrecognized_symbol(code);
      return Err(format!("Unidentified symbol {symbol}"));
  
    }
  
    return Ok(tokens);
  }

  fn lex_keyword(code: &str) -> (bool, Token, &str) {
    enum StateMachine {
        Start,
        Func,
        IntIf,
        Print,
        ReadReturn,
        While,
        Else,
        Break,
        Continue,
        Keyword,
    }

    let mut success = false;
    let mut state = StateMachine::Start;
    let mut index = 0;
    let mut temp = "";

    for letter in code.chars() {
        match state {
          StateMachine::Start => {
            match letter {
              'f' => { 
                state = StateMachine::Func;
                success = true;
                //index += 1;
              },
              'i' => {  
                state = StateMachine::IntIf;
                success = true;
                //index += 1;
              },
              'p' => {  
                state = StateMachine::Print;
                success = true;
                //index += 1;
              },
              'r' => {  
                state = StateMachine::ReadReturn;
                success = true;
                //index += 1;
              },
              'w' => { 
                state = StateMachine::While;
                success = true;
                //index += 1;
              },
              'e' => {  
                state = StateMachine::Else;
                success = true;
                //index += 1;
              },
              'b' => {  
                state = StateMachine::Break;
                success = true;
                //index += 1;
              },
              'c' => {  
                state = StateMachine::Continue;
                success = true;
                //index += 1;
              },
              _ => {  
                return (false, Token::NotToken, "");
              },
            }
          }

          StateMachine::Func => {
            if code.len() > 3{
              temp = &code[..3];
            } else {
              temp = "";
            }
            
            if temp == "func" {
              state = StateMachine::Keyword;
              success = true;
              index = 3;
            } else {
              state = StateMachine::Keyword;
              success = false;
            }
          }
          
          StateMachine::IntIf => { //todo not working
            if code.len() < 2 {
              state = StateMachine::Keyword;
              success = false;
            } else if &code[..1] == "if" {
              state = StateMachine::Keyword;
              success = true;
              index += 1;
            } else if code.len() < 3 {
              state = StateMachine::Keyword;
              success = false;
            } else if &code[..2] == "int" {
              state = StateMachine::Keyword;
              success = true;
              index = 2;
            } else {
              state = StateMachine::Keyword;
              success = false;
            }
          }

          StateMachine::Print => {
            if code.len() > 4 {
              temp = &code[..4];
            } else {
              temp = "";
            }
            if temp == "print" {
              state = StateMachine::Keyword;
              success = true;
              index = 4;
            } else {
              state = StateMachine::Keyword;
              success = false;
            }
          }

          StateMachine::ReadReturn => {
            if code.len() < 4{
              state = StateMachine::Keyword;
              success = false;
            } else if &code[..3] == "read"{
              state = StateMachine::Keyword;
              success = true;
              index = 3;
            } else if code.len() < 6 {
              state = StateMachine::Keyword;
              success = false;
            } else if &code[..5] == "return" {
              temp = &code[..5];
              state = StateMachine::Keyword;
              success = true;
              index = 5;
            } else {
              state = StateMachine::Keyword;
              success = false;
            }
          }

          StateMachine::While => {
            if code.len() > 4{
              temp = &code[..4];
            } else{
              temp = "";
            }
            if temp == "while" {
              state = StateMachine::Keyword;
              success = true;
              index = 4;
            } else {
              state = StateMachine::Keyword;
              success = false;
            }
          }

          StateMachine::Else => {
            if temp.len() > 3{
              temp = &code[..3];
            } else{
              temp = ""
            }
            
            if temp == "else" {
              state = StateMachine::Keyword;
              success = true;
              index = 3;
            } else {
              state = StateMachine::Keyword;
              success = false;
            }
          }

          StateMachine::Break => {
            if code.len() > 4{
              temp = &code[..4];
            } else{
              temp = "";
            }
            if temp == "break" {
              state = StateMachine::Keyword;
              success = true;
              index = 4;
            } else {
              state = StateMachine::Keyword;
              success = false;
            }
          }

          StateMachine::Continue => {
            if code.len() > 7{
              temp = &code[..7];
            } else{
              temp = "";
            }
            if temp == "ontinue" {
              state = StateMachine::Keyword;
              success = true;
              index = 7;
            } else {
              state = StateMachine::Keyword;
              success = false;
            }
          }

          StateMachine::Keyword => {
            if success == true {
              let token = &code[..index];
              return(true, create_identifier(token), &code[index..]);
            } else {
              return (false, Token::NotToken, "");
            }
          }
        }
    }

    if success == true { //if we are creating a token here -> we can assume that the token spans the input
      return (true, create_identifier(code), "");
    } else {
      return (false, Token::NotToken, "");
    }
  }

  fn lex_space(code: &str) -> (bool, &str) {
    for letter in code.chars() {
      if letter.is_whitespace() {
        return (true, &code[1..]);
      } else {
        return (false, code);
      }
    }
    return (false, code);
  }

  fn lex_number(code: &str) -> (bool, Token, &str) {
    enum StateMachine {
      Start,
      Number,
    }
  
    let mut success = false;
    let mut state = StateMachine::Start;
    let mut index = 0;
    for letter in code.chars() {
      match state {
      StateMachine::Start => {
        if letter >= '0' && letter <= '9' {
          state = StateMachine::Number;
          success = true;
          index += 1;
        } else {
          return (false, Token::NotToken, "");
        }
      }
  
      StateMachine::Number => {
        if letter >= '0' && letter <= '9' {
          state = StateMachine::Number;
          success = true;
          index += 1;
        } else if (letter >= '?' && letter <= 'Z') || (letter >= '^' && letter <= 'z') || letter >= '$'  && letter <= '\'' || letter == '"' || letter == '.' || letter == '\\' || letter == ':' || letter == '|' || letter == '~' {

          return (false, Token::NotToken, "");
        } else {
          let num = code[..index].parse::<i32>().unwrap();
          return (true, Token::Num(num), &code[index..]);
        }
      }
  
      }
    }
  
    if success == true {
      let num: i32 = code.parse::<i32>().unwrap();
      return (true, Token::Num(num), "");
    } else {
      return (false, Token::NotToken, "");
    }
  }

  fn lex_identifier(code: &str) -> (bool, Token, &str) {
    enum StateMachine {
      Start,
      Ident,
    }
  
    let mut success = false;
    let mut state = StateMachine::Start;
    let mut index = 0;
    for letter in code.chars() {
      match state {
      StateMachine::Start => {
        if (letter >= 'a' && letter <= 'z') || (letter >= 'A' && letter <= 'Z'){
          state = StateMachine::Ident;
          success = true;
          index += 1;
        } else {
          return (false, Token::NotToken, "");
        }
      }
  
      StateMachine::Ident => {
        if (letter >= 'A' && letter <= 'Z') || (letter >= 'a' && letter <= 'z') || (letter >= '0' && letter <= '9') || letter == '_' {
          state = StateMachine::Ident;
          success = true;
          index += 1;
        } else {
          let token = &code[..index];
          return (true, create_identifier(token), &code[index..]);
        }
      }
  
      }
    }
  
    if success == true {
      return (true, create_identifier(code), "");
    } else {
      return (false, Token::NotToken, "");
    }
  }

  fn unrecognized_symbol(code: &str) -> &str {
    enum StateMachine {
      Start,
      Symbol,
    }
  
    let mut state_machine = StateMachine::Start;
    let mut index = 0;
    for letter in code.chars() {
      match state_machine {
      StateMachine::Start => {
        state_machine = StateMachine::Symbol;
        index += 1;
      } 
      
      StateMachine::Symbol => {
        if letter.is_whitespace() {
          return &code[..index];
        } else {
          index += 1;
        }
      }
  
      }
    }
    return &code[..index];
  } 
fn create_identifier(code: &str) -> Token {
    match code {
    "func" => Token::Func,
    "return" => Token::Return,
    "int" => Token::Int,
    "print" => Token::Print,
    "read" => Token::Read,
    "while" => Token::While,
    "if" => Token::If,
    "else" => Token::Else,
    "break" => Token::Break,
    "continue" => Token::Continue,
    "(" => Token::LeftParen,
    ")" => Token::RightParen,
    "{" => Token::LeftCurly,
    "}" => Token::RightCurly,
    "[" => Token::LeftBracket,
    "]" => Token::RightBracket,
    "," => Token::Comma,
    ";" => Token::Semicolon,
    "+" => Token::Plus,
    "-" => Token::Subtract,
    "*" => Token::Multiply,
    "/" => Token::Divide,
    "%" => Token::Modulus,
    "=" => Token::Assign,
    "<" => Token::Less,
    "<=" => Token::LessEqual,
    ">" => Token::Greater,
    ">=" => Token::GreaterEqual,
    "==" => Token::Equality,
    "!=" => Token::NotEqual,
     _ => Token::Ident(String::from(code))
    }
}

static mut VAR_NUM: i64 = 0;

fn create_temp() -> String {
    unsafe {
        VAR_NUM += 1;
        format!("_temp{}", VAR_NUM)
    }
}

static mut IF_NUM: i32 = 0;

fn create_num() -> i32 {
  unsafe {
    IF_NUM += 1;
    return IF_NUM;
  }
}

fn peek<'a>(tokens: &'a Vec<Token>, index: usize) -> Option<&'a Token> {
  if index < tokens.len() {
      return Some(&tokens[index])
  } else {
      return None
  }
}

fn peek_result<'a>(tokens: &'a Vec<Token>, index: usize) -> Result<&'a Token, String> {
  if index < tokens.len() {
      return Ok(&tokens[index])
  } else {
      return Err(String::from("expected a token, but got nothing"))
  }
}

fn next<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Option<&'a Token> {
  if *index < tokens.len() {
      let ret = *index;
      *index += 1;
      return Some(&tokens[ret])
  } else {
      return None
  }
}

fn next_result<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<&'a Token, String> {
  if *index < tokens.len() {
      let ret = *index;
      *index += 1;
      return Ok(&tokens[ret])
  } else {
      return Err(String::from("expected a token, but got nothing"))
  }
}

// parse programs with multiple functions
// loop over everything, outputting generated code.
fn parse_program(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
  let mut generated_code = String::from("");
  let mut func_table: Vec<String> = vec![];
  loop {
      match parse_function(tokens, index, &mut func_table)? {
      None => {
          break;
      }
      Some(code) => {
        generated_code += &code;
      }
      }
  }
  
  if !in_table(&func_table, &format!("main")){
    return Err(format!("missing 'main' function"));
  }

  return Ok(generated_code);
}

fn in_table(table: &Vec<String>, symbol: &String) -> bool {
  for s in table{
    if s.eq(symbol) {
      return true;
    }
  }
  false
}

// parse function such as:
// func main(int a, int b) {
//    # ... statements here...
//    # ...
// }
// a loop is d  one to handle statements.

fn parse_function(tokens: &Vec<Token>, index: &mut usize, func_table: &mut Vec<String>) -> Result<Option<String>, String> {
  
  let mut symbol_table: Vec<String> = vec![];
  let mut arr_table: Vec<String> = vec![];
  match next(tokens, index) {
    None => {
        return Ok(None);
    }
    Some(token) => {
        if !matches!(token, Token::Func) {
            return Err(String::from("functions must begin with func"));
        }
    }

  }
  
  let func_ident = match next_result(tokens, index)? {
  Token::Ident(func_ident) => {func_ident},
  _  => {return Err(String::from("functions must have a function identifier"));}
  };

  if in_table(func_table, func_ident){
    return Err(format!("Error: Function {func_ident} already declared"));
  }
  func_table.push(func_ident.clone());

  if !matches!( next_result(tokens, index)?, Token::LeftParen) {
      return Err(String::from("expected '(' "));
  }

  let mut code = format!("%func {} (", func_ident);
  let mut params: Vec<String> = vec![];

  loop {
     match next_result(tokens, index)? {
      Token::RightParen => {
          break;
      }
      Token::Int => {
          match next_result(tokens, index)? {
            Token::Ident(param) => {
                if in_table(&symbol_table, param) {
                  return Err(format!("Found duplicate variable {param}"));
                }
                symbol_table.push(param.clone());
                code += &format!("%int {}", param);
                params.push(param.clone());
                match peek_result(tokens, *index)? {
                  Token::Comma => {
                    code += &format!(", ");
                    *index += 1;
                  }
                  Token::RightParen => {}
                  _ => {
                      return Err(String::from("expected ',' or ')' "));
                  }
                }
            }
            _ => {
                return Err(String::from("expected ident function parameter"));
            }
          }
      }

      _ => {
          return Err(String::from("expected 'int' keyword or ')' token"));
      }
     }
  }
  code += &format!(")\n");


  if !matches!(next_result(tokens, index)?, Token::LeftCurly) {
      return Err(String::from("expected '{'"));
  }

  loop {
      match parse_statement(tokens, index, &mut symbol_table, func_table, &mut arr_table, false, &mut 0)? {
      None => {
          break;
      }
      Some(statement) => {
        code += &statement;
      }
      }
  }
  code += "%endfunc\n";

  if !matches!(next_result(tokens, index)?, Token::RightCurly) {
    println!("{:?}", tokens[*index]);
    return Err(String::from("expected '}'"));
  }

  return Ok(Some(code));
}


fn parse_statement(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<String>, func_table: &mut Vec<String>, arr_table: &mut Vec<String>, in_loop: bool, nesting_level: &mut i32) -> Result<Option<String>, String> {
  match peek(tokens, *index) {
    None => {
        return Ok(None);
    }
    Some(token) => {
        let ast_node: Option<String>; //CHANGED DECLARATION NONE -> UNINITIALIZED ; REMOVED MUT
        match token {
            Token::RightCurly => {
                return Ok(None);
            }

            Token::Break => {
                let mut code: String = Default::default();
                if !in_loop {
                return Err(String::from("Tried to break but not in loop"));
                }
                *index += 1;
                match next_result(tokens, index)? {

                Token::Semicolon => {}
                _ => {
                  return Err(String::from("expect ';' closing statement"));
                }
                }
                code += &format!("%jmp :endloop{}\n", nesting_level);
                ast_node = Some(code);
            }

            Token::Int => {
              let code = parse_declaration(tokens, index, symbol_table, func_table, arr_table)?; //removed string new declaration
              ast_node = Some(code);
            }

            Token::Ident(ident) => {
                let mut code = String::new();
                *index += 1;
                match peek_result(tokens, *index)?{
                  Token::LeftBracket => {
                    *index += 1;
                    let position = parse_expression(tokens, index, symbol_table, func_table, arr_table, &mut code)?;
                    match next_result(tokens, index)?{
                      Token::RightBracket =>{
                        match next_result(tokens, index)? {
                          Token::Assign =>{
                            let expr = parse_expression(tokens, index, symbol_table, func_table, arr_table, &mut code)?;
                            code += &format!("%mov [{} + {}], {}\n", ident, position, expr);
                            match next_result(tokens, index)? { //after we have parsed the RHS expression we should close it off with a semi
                              Token::Semicolon => {}
                              _=> {
                                return Err(String::from("expect ';' closing statement 0"));
                              }
                            }
                          }
                          _ => {
                            return Err(String::from("expected '=' opperator"));
                          }
                        }
                      }
                      _ => {
                        return Err(String::from("expected ']' "))
                      }
                    }
                  }
                  Token::Assign => {
                    *index += 1;
                    let exp = parse_expression(tokens, index, symbol_table, func_table, arr_table, &mut code)?;
                    code += &format!("%mov {}, {}\n", ident, exp);
                    match next_result(tokens, index)? {
                      Token::Semicolon => {}
                      _ => {
                          return Err(String::from("expect ';' closing statement1"));
                      }
                    }
                  }
                    _ => {
                    return Err(String::from("expect '[' or '='"));
                  }
                }
              ast_node = Some(code);
            }

            Token::Return => {
                *index += 1;
                let mut code = String::new();
                let exp = parse_expression(tokens, index, symbol_table, func_table, arr_table, &mut code)?;
                code += &format!{"%ret {}\n", exp};
                match next_result(tokens, index)? {
                    Token::Semicolon => {}
                    _ => {
                    return Err(String::from("expect ';' closing statement1"));
                    }
                }
                ast_node = Some(code);
            }

            Token::Print => {
                *index += 1;
                if !matches!(next_result(tokens, index)?, Token::LeftParen) {
                    return Err(String::from("expect '(' closing statement"));
                }
                let mut code: String = Default::default();
                let exp = parse_expression(tokens, index, symbol_table, func_table, arr_table, &mut code)?;
                let dest = create_temp();
                code += &format!("%int {dest}\n%mov {dest}, {exp}\n");
                code += &format!("%out {}\n", dest);
                if !matches!(next_result(tokens, index)?, Token::RightParen) {
                    return Err(String::from("expect ')' closing statement"));
                }
                match next_result(tokens, index)? {
                    Token::Semicolon => {}
                    _ => {
                    return Err(String::from("expect ';' closing statement1"));
                    }
                }
                ast_node = Some(code);
            }

            Token::Read => {
                *index += 1;
                if !matches!(next_result(tokens, index)?, Token::LeftParen) {
                    return Err(String::from("expect '(' closing statement"));
                }
                let mut code: String = Default::default();
                let term = parse_term(tokens, index, symbol_table, func_table, arr_table, &mut code)?;
                if !matches!(next_result(tokens, index)?, Token::RightParen) {
                    return Err(String::from("expect ')' closing statement"));
                }
                code += &format!("%input {}\n", term);

                match next_result(tokens, index)? {
                    Token::Semicolon => {}
                    _ => {
                    return Err(String::from("expect ';' closing statement1"));
                    }
                }
                ast_node = Some(code);
            }
            
            Token::Continue => {
                let mut code: String = Default::default();
                if !in_loop {
                    return Err(String::from("Tried to continue but not in loop"));
                }
                *index += 1;
                match next_result(tokens, index)? {
                  Token::Semicolon => {}
                  _ => {
                      return Err(String::from("expect ';' closing statement"));
                  }
                }
                code += &format!("%jmp :loopbegin{}\n", nesting_level);
                ast_node = Some(code);
            }

            Token::While => {
                              
                // We see a while keyword, create loopbegin label, maybe keep a counter outside for the number of loops, before while loop
                // parse bool should return out code variable updated with the value of the new IM generation
                // perform break evaluation ------------------?
                // match -> go into while loop searching for statements
                // come out of while loop -> end
            
                *index += 1; //consumed while keyword -> validate while statement
                 //increment the nesting level to indicate that we've entered a while loop
                let mut local_nesting_level = create_num();
                let mut code = format!(":loopbegin{}\n", local_nesting_level); 

                
                parse_bool(tokens, index, symbol_table, func_table, arr_table, &mut code, &mut local_nesting_level, "while".to_string())?;

                match next_result(tokens, index)? {
                  Token::LeftCurly => {}
                  _ => {
                      return Err(String::from("expected '{' operator : 'parse statement while_loop block'"));
                  }
                }
                loop { //if statement is 'while bool {' search for statements
                  match parse_statement(tokens, index, symbol_table, func_table, arr_table, true, &mut local_nesting_level)? {
                    None => {
                      break;
                    }
                    Some(stmt) => {
                      code += &stmt;
                    }
                  }
                }

                code += &format!("%jmp :loopbegin{}\n", local_nesting_level);
                code += &format!(":endloop{}\n", local_nesting_level);

                match next_result(tokens, index)? {
                  Token::RightCurly => {}
                  _ => {
                    return Err(String::from("expected '}' operator"));
                  }
                }
                ast_node = Some(code);
            }

            Token::If => {
                *index += 1; //consumed if keyword -> validate if statement
                let mut ifNumber = create_num();
                let mut code: String = Default::default();
                // Implement bool in phase 4
                parse_bool(tokens, index, symbol_table, func_table, arr_table, &mut code, &mut ifNumber, "if".to_string())?;
                code += &format!("%jmp :else{}\n", ifNumber);
                code += &format!(":iftrue{}\n", ifNumber);

                match next_result(tokens, index)? {
                  Token::LeftCurly => {}
                  _ => {
                      return Err(String::from("expected '{' operator1"));
                  }
                }
                loop { //we've consumed 'if bool {' now we can search for statements
                  let mut local_nesting_level = *nesting_level;
                  match parse_statement(tokens, index, symbol_table, func_table, arr_table, in_loop, &mut local_nesting_level)? {
                    None => {
                      break;
                    }
                    Some(stmt) => {
                      code += &stmt;
                    }
                  }
                }
                match next_result(tokens, index)? {
                  Token::RightCurly => {}
                  _ => {
                    return Err(String::from("expected '}' operator1"));
                  }
                }
                code += &format!("%jmp :endif{}\n", ifNumber);
                code += &format!(":else{}\n", ifNumber);
                match peek_result(tokens, *index)? { //ensure statement closes THEN, peek ahead to see if end of input for if else
                  Token::Else => {},
                  _ => {
                    code += &format!(":endif{}\n", ifNumber);
                    ast_node = Some(code);
                    return Ok(ast_node);
                  }
                };

                if!matches!(next_result(tokens, index)?, Token::LeftCurly) { // if 'else' keyword consumed, then repeat validation for 'if' body as seen above
                    *index += 1;
                } else {
                    return Err(String::from("expected '{' operator3"));
                }

                loop {
                  match parse_statement(tokens, index, symbol_table, func_table, arr_table, in_loop, nesting_level)? {
                      None => {
                      break;
                      }
                      Some(stmt) => {
                        code += &stmt;
                      }
                  }
                }

                match next_result(tokens, index)? {
                    Token::RightCurly => {}
                    _ => {
                        return Err(String::from("expected '}' operator2"));
                    }
                }
                code += &format!(":endif{}\n", ifNumber);
                ast_node = Some(code);
            }

            _ => {
                println!("\nStatement");
                println!("{:?}", tokens[*index]);
                println!("Prev 3 tokens: {:?}, {:?}, {:?}", tokens[*index-1], tokens[*index-2], tokens[*index-3]);
                return Err(String::from("invalid statement."));
            }
        }
        return Ok(ast_node);
    }
  }
}


fn parse_expression(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<String>, func_table: &mut Vec<String>, arr_table: &mut Vec<String>, code: &mut String) -> Result<String, String> {
  let mut term = parse_term(tokens, index, symbol_table, func_table, arr_table, code)?;
  let dest = create_temp();
  *code += &format!("%int {} \n", dest); //uncessary temp variable declaration?

  loop{
    match peek_result(tokens, *index)? {
        Token::Plus => {
            *index += 1;
            let exp = parse_expression(tokens, index, symbol_table, func_table, arr_table, code)?;
            // I have the term, and the expression. Now I need to create a temp, and add the term and expression into it and return
            *code += &format!("%add {}, {}, {}\n", dest, term, exp);
        },
        Token::Subtract => {
            *index += 1;
            let exp = parse_expression(tokens, index, symbol_table, func_table, arr_table, code)?;
            *code += &format!("%sub {}, {}, {}\n", dest, term, exp);
        },
        Token::Multiply => {
            *index += 1;
            let next_term = parse_term(tokens, index, symbol_table, func_table, arr_table, code)?;
            *code += &format!("%mult {}, {}, {}\n", dest, term, next_term);
        },
        Token::Divide => {
            *index += 1;
            let next_term = parse_term(tokens, index, symbol_table, func_table, arr_table, code)?;
            *code += &format!("%div {}, {}, {}\n", dest, term, next_term);
        },
        Token::Modulus => {
            *index += 1;
            let next_term = parse_term(tokens, index, symbol_table, func_table, arr_table, code)?;
            *code += &format!("%mod {}, {}, {}\n", dest, term, next_term);
        }
        _ => {
            return Ok(term);
        }
      }
      term = dest.clone();
  };
}

fn parse_bool(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<String>, func_table: &mut Vec<String>, arr_table: &mut Vec<String>, code: &mut String, nesting_level: &mut i32, _stmt_type: String) -> Result<(), String> {
  let expression = parse_expression(tokens, index, symbol_table, func_table, arr_table, code)?;
  let dest = create_temp();
  *code += &format!("%int {} \n", dest); //uncessary temp variable declaration?

  match peek_result(tokens, *index)? {
    Token::Less => {
      *code += &format!("%lt {}, {}, ", dest, expression);
    }
    Token::LessEqual => {
      *code += &format!("%le {}, {}, ", dest, expression);
    },
    Token::Equality => {
      *code += &format!("%eq {}, {}, ", dest, expression);
    },
    Token::NotEqual => {
      *code += &format!("%neq {}, {}, ", dest, expression);
    },
    Token::GreaterEqual => {
      *code += &format!("%ge {}, {}, ", dest, expression);
    },
    Token::Greater => {
      *code += &format!("%gt {}, {}, ", dest, expression);
    },

    _ => {
      return Ok(());
    }

  };

  *index += 1;
  let term = parse_term(tokens, index, symbol_table, func_table, arr_table, code)?;
  *code += &format!("{}\n", term);
  // *code += &format!("%branch_ifn {}, :endloop{}\n", dest, nesting_level);

  if _stmt_type == "while" {
    *code += &format!("%branch_ifn {}, :endloop{}\n", dest, nesting_level);
  } else {
    *code += &format!("%branch_if {}, :iftrue{}\n", dest, nesting_level);
  }

  return Ok(());
}

fn parse_declaration(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<String>, func_table: &mut Vec<String>, arr_table: &mut Vec<String>) -> Result<String, String> {
  match next_result(tokens, index)? {
        Token::Int => {
          let mut code: String = Default::default();
            match next_result(tokens, index)? {
                Token::LeftBracket => {  // Array size is declared after 'int'
                    let size = parse_term(tokens, index, symbol_table, func_table, arr_table, &mut code)?;  // Parses the size of the array
                    if !matches!(next_result(tokens, index)?, Token::RightBracket) {
                        return Err(String::from("expect ']' closing array size declaration"));
                    }
                    match next_result(tokens, index)? {
                        Token::Ident(name) =>{
                            if in_table(&arr_table, name) {
                                return Err(format!("Found a duplicate array variable {name}"));
                            }
                            code += &format!("%int[] {name}, {size} \n");
                            arr_table.push(name.to_string());
                        },
                        _ => {return Err(String::from("expected identifier for array name"));}
                    }

                    match next_result(tokens, index)? {
                      Token::Semicolon => {
                        return Ok(code);
                      }
                      _ => {
                        return Err(String::from("expect ';' closing statement d"));
                      }
                    }
                    

                },
                Token::Ident(name) => {  // Normal int variable declaration
                    if in_table(symbol_table, name){
                        return Err(String::from("Variable already declared"));
                    }
                    code += &format!("%int {name}\n");
                    match next_result(tokens, index)? {
                      Token::Assign => {
                        let exp = parse_expression(tokens, index, symbol_table, func_table, arr_table, &mut code)?;
                        code += &format!("%mov {name}, {exp}\n");
                        match next_result(tokens, index)?{
                          Token::Semicolon => {}
                          _ => {
                            return Err(String::from("expect ';' closing statement"));
                          }
                        }
                      }
                      Token::Semicolon => {}
                      _ => {
                        println!("{:?}", tokens[*index]);
                        return Err(String::from("expect ';' closing statement 2"));
                      }
                    }
                    Ok(code)
                },
                _ => Err(String::from("expected '[' for array declaration or identifier for int variable")),
            }
        },
        _ => Err(String::from("invalid declaration, expected 'int' type")),
    }
}


// a term is either a Number or an Identifier.
fn parse_term(tokens: &Vec<Token>, index: &mut usize, symbol_table: &mut Vec<String>, func_table: &mut Vec<String>, arr_table: &mut Vec<String>, code: &mut String) -> Result<String, String> {
  match next_result(tokens, index)? {
    Token::Ident(ident) => {
        match peek_result(tokens, *index)? {
            Token::LeftBracket => {
                *index += 1;
                if !in_table(arr_table, ident) {
                    return Err(format!("Array {ident} not initialized"));
                }
                let size = parse_term(tokens, index, symbol_table, func_table, arr_table, code)?;  // Parses the size of the array
                if !matches!(next_result(tokens, index)?, Token::RightBracket) {
                    return Err(String::from("expect ']' closing array size declaration"));
                }
                let dest = create_temp();
                *code += &format!("%int {dest}\n%mov {dest}, [{ident} + {size}]\n");
                return Ok(format!("{}", dest));
            }

            Token::LeftParen => {
                *index += 1;
                if !in_table(func_table, ident) {
                    return Err(format!("Function {ident} not initialized"));
                }
                let dest = create_temp();
                *code += &format!("%int {dest}\n");
 
                let mut args_code = String::new();
                loop {
                    match peek_result(tokens, *index)? {
                        Token::RightParen => {
                            *index += 1;
                            break;
                        }
                        _ => {
                            let exp = parse_expression(tokens, index, symbol_table, func_table, arr_table, code)?;
                            args_code += &exp;
                            match peek_result(tokens, *index)? {
                                Token::Comma => {
                                 *index += 1;
                                 args_code += ", ";
                                }
                                Token::RightParen => {}
                                _ => {
                                    return Err(String::from("expected ',' or ')'"))
                                }
                            }
                        }
                    }
                }
                *code += &format!("%call {}, {}({})\n", dest, ident, args_code);
                return Ok(dest);
            }
            _ => {
              return Ok(format!("{}", &ident));
            }
        }
    }
    Token::Num(num) => {
        return Ok(format!("{}", num));
    }
    Token::LeftParen => {
        let exp = parse_expression(tokens, index, symbol_table, func_table, arr_table, code)?;
        if !matches!(next_result(tokens, index)?, Token::RightParen) {
            return Err(String::from("expected ')'"));
        }
        return Ok(exp);
    }
    _ => {
        println!("{:?}", tokens[*index]);
        return Err(String::from("invalid expression"));
    }
  }
}