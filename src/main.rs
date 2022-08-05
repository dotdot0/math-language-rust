#![allow(dead_code, unused)]
mod token;
use crate::token::tokenize;

fn main(){
    let num: i32 = 46454645;
   let tokens = tokenize("x + z - y * j / 9;").unwrap();
   for token in tokens{
       println!("{:?}", token)
   }
   println!("Hello World!");
   println!("Tokenizer")
}

