#![allow(dead_code, unused)]

use std::io;
use std::thread::current;
use std::any;

#[derive(Debug)]
pub enum Error{
    NotValidToken
}

#[derive(Debug)]
pub enum TokenType{
    Start,
    Add,
    Multiply,
    Divide,
    Subtract,
    End,
    Integer,
    WhiteSpace,
    Identifier,
    Lparen,
    Rparen,
    Assign
}

#[derive(Debug)]
pub struct Token{
    pub ttype: TokenType,
    pub value: String
}

pub fn tokenize(code: &String) -> Result<Vec<Token>, Error>{
    let mut current_token: Token = Token{
        ttype: TokenType::Start,
        value: String::from("codestart")
    };
    let mut tokens = Vec::new();
    let mut column: i32 = 0;
    for ch in code.chars(){
        if ch == '+'{
            current_token = Token{
                ttype: TokenType::Add,
                value: String::from("+")
            };  
            tokens.push(current_token);
            column += 1;
            println!("{column}")
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
        else if ch.is_whitespace(){
           current_token = Token{
                  ttype: TokenType::WhiteSpace,
                  value: String::from("WhiteSpace")
           };
           tokens.push(current_token)
        }
        else if ch.is_alphabetic(){
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
        else if ch.is_numeric(){
            current_token = Token{
                ttype : TokenType::Integer,
                value : String::from(ch)
            };
            tokens.push(current_token)
        }
        else if ch == ')'{
            current_token = Token{
                ttype : TokenType::Rparen,
                value : String::from(")")
            }
        }
        else if ch == '('{
            current_token = Token{
                ttype : TokenType::Lparen,
                value : String::from("(")
            };
            tokens.push(current_token)
        }
        else if ch == '='{
            current_token = Token{
                ttype : TokenType::Assign,
                value : String::from("=")
            };
            tokens.push(current_token)
        }
    }
    Ok(tokens)
}

