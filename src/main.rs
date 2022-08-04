#![allow(dead_code, unused)]

use std::thread::current;

#[derive(Debug)]
enum Error{
    NotValidToken
}

#[derive(Debug)]
enum TokenType{
    Start,
    Add,
    Multiply,
    Divide,
    Subtract,
    End,
    Value,
    WhiteSpace
}

#[derive(Debug)]
struct Token{
    ttype: TokenType,
    value: String
}

fn tokenize(code: &str) -> Result<Vec<Token>, Error>{
    let mut current_token: Token = Token{
        ttype: TokenType::Start,
        value: String::from("codestart")
    };
    let mut tokens = Vec::new();
    for ch in code.chars(){
        if ch == '+'{
            current_token = Token{
                ttype: TokenType::Add,
                value: String::from("+")
            };  
            tokens.push(current_token);
        }
        else if ch == '-' {
            current_token = Token{
                ttype: TokenType::Subtract,
                value: String::from("-")
            };
            tokens.push(current_token)
        }
        else if ch == '*' {
            current_token = Token{
                ttype: TokenType::Multiply,
                value: String::from("*")
            };
            tokens.push(current_token)
        }
        else if ch == '/' {
           current_token = Token{
               ttype : TokenType::Divide,
               value : String::from("/")
        };
           tokens.push(current_token)
        }
        else if ch == ' ' {
           current_token = Token{
                  ttype: TokenType::WhiteSpace,
                  value: String::from("WhiteSpace")
           };
           tokens.push(current_token)
        }
        else if ch == ';'{
            current_token = Token{
                ttype: TokenType::End,
                value: String::from(";")
            };
            tokens.push(current_token)
        }
        else {
            current_token = Token{
                ttype : TokenType::Value,
                value : String::from("value")
            };
            tokens.push(current_token)
        }
    }
    Ok(tokens)
}

fn main(){
   let tokens = tokenize("x + y;").unwrap();
   for token in tokens{
       println!("{:?}", token)
   }
   println!("Tokenizer")
}

