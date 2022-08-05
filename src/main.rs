#![allow(dead_code, unused)]


use std::io;
use std::thread::current;
use std::any;

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
    Integer,
    WhiteSpace,
    Identifier
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
        else if ch >= 'a' && ch <= 'z' || ch >= 'A' && ch <= 'Z'{
            current_token = Token{
                ttype: TokenType::Identifier,
                value: String::from(ch)
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
        else if ch >= '0' && ch <= '9'{
            current_token = Token{
                ttype : TokenType::Integer,
                value : String::from(ch)
            };
            tokens.push(current_token)
        }
    }
    Ok(tokens)
}

fn main(){
    let num: i32 = 46454645;
   let tokens = tokenize("x + z - y * j / 9;").unwrap();
   for token in tokens{
       println!("{:?}", token)
   }
   println!("Hello World!");
   println!("Tokenizer")
}

