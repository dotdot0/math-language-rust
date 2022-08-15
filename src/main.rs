#![allow(dead_code, unused)]
mod token;
mod parser;
use std::io::{self, Write};
use crate::token::{tokenize, Token, TokenType};
use crate::parser::parse; 

fn main(){
   loop{
     let mut code: String = String::new();
     print!(">");
     let _ = io::stdout().flush();
     io::stdin().read_line(&mut code).expect("Unable To Read Source Code");
     if code == " "{
       continue;
     }
     else if code.trim() == "q"{
       break;
     }
     else{
       let mut tokens = tokenize(&code.trim().to_string()).unwrap();
       println!("{:?}", parse(tokens).unwrap())
       }
     }
   }

