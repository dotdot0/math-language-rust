#![allow(dead_code, unused)]
mod token;
use crate::token::{tokenize, Token, TokenType};

fn main(){
   let maintoken = Token{
     ttype : TokenType::Multiply,
     value : String::from("*")
   };
   let tokens = tokenize("6 + 9;").unwrap();
   for token in tokens{
       println!("{:?}", token)
   }
   println!("{:?}", maintoken);
   
}

