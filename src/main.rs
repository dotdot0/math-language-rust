#![allow(dead_code, unused)]
mod token;
use std::io;
use crate::token::{tokenize, Token, TokenType};

fn main(){
   loop{
     let mut code: String = String::new();
     println!(">");
     io::stdin().read_line(&mut code).expect("Unable To Read Source Code");
     if code == " "{
       continue;
     }
     else if code == ""{
       break;
     }
     else{
       let tokens = tokenize(&code).unwrap();
       for token in tokens{
         println!("{:?}", token)
       }
     }
   }
   
}

