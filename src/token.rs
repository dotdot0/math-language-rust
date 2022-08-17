#![allow(dead_code, unused)]

use std::io;
use std::thread::current;
use std::any;

#[derive(Debug, PartialEq)]
pub enum Error{
    NotValidToken
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType{
    Start,
    Add,
    Multiply,
    Divide,
    Subtract,
    End,
    Number ,
    WhiteSpace,
    Lparen,
    Rparen,
    Assign
}

#[derive(Debug,Clone)]
pub struct Token{
    pub ttype: TokenType,
    pub value: String
}

impl Token{
    pub fn parse_number(&self) -> i32{
        let mut num: Option<i32> = None;
        if self.ttype == TokenType::Number{
            num = Some(self.value.parse().unwrap())
        }
        num.unwrap() 
    }
}

pub fn tokenize(code: &String) -> Result<Vec<Token>, Error>{
    let mut current_token: Option<Token> = None;
    let mut tokens = Vec::new();
    for ch in code.chars(){
        if ch == '+'{
            current_token = Some(Token{
                ttype: TokenType::Add,
                value: String::from("+")
            });  
            tokens.push(current_token.unwrap());
        }
        else if ch == '-' {
            current_token = Some(Token{
                ttype: TokenType::Subtract,
                value: String::from("-") 
            });
            tokens.push(current_token.unwrap())
        }
        else if ch == '*' {
            current_token = Some(Token{
                ttype: TokenType::Multiply,
                value: String::from("*")
            });
            tokens.push(current_token.unwrap())
        }
        else if ch == '/' {
           current_token = Some(Token{
               ttype : TokenType::Divide,
               value : String::from("/")
        });
           tokens.push(current_token.unwrap())
        }
        else if ch.is_whitespace(){
           current_token = Some(Token{
                  ttype: TokenType::WhiteSpace,
                  value: String::from("WhiteSpace")
           });
        }
        else if ch == ';'{
            current_token = Some(Token{
                ttype: TokenType::End,
                value: String::from(";")
            });
            tokens.push(current_token.unwrap())
        }
        else if ch.is_numeric(){
            current_token = Some(Token{
                ttype : TokenType::Number,
                value : String::from(ch)
            });
            tokens.push(current_token.unwrap())
        }
        else if ch == ')'{
            current_token = Some(Token{
                ttype : TokenType::Rparen,
                value : String::from(")")
            });
            tokens.push(current_token.unwrap())
        }
        else if ch == '('{
            current_token = Some(Token{
                ttype : TokenType::Lparen,
                value : String::from("(")
            });
            tokens.push(current_token.unwrap())
        }
        else if ch == '='{
            current_token = Some(Token{
                ttype : TokenType::Assign,
                value : String::from("=")
            });
            tokens.push(current_token.unwrap())
        }
        else {
            panic!("Invalid Token")
        }
    }
    Ok(tokens)
}


