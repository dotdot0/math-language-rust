#![allow(dead_code, unused)]
mod token;
use crate::token::{tokenize, Token};

fn main(){
   let tokens = tokenize("6 + 9;").unwrap();
   for token in tokens{
       println!("{:?}", token)
   }
   println!("Tokenizer");
   
}

