#![allow(dead_code, unused)]
mod token;
use crate::token::tokenize;

fn main(){
   let tokens = tokenize("x - z + 9 * 8;").unwrap();
   for token in tokens{
       println!("{:?}", token)
   }
   println!("Hello World!");
   println!("Tokenizer")
}

