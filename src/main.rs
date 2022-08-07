#![allow(dead_code, unused)]
mod token;
use std::io;
use crate::token::{tokenize, Token, TokenType};

fn main(){
   loop{
     println!(">");
     let mut code = String::new();
     io::stdin().read_line(&mut code).expect("Unable To Read Code");
     if code == " "{
       println!("Please Enter Code.")
     }
     else if code == "quit"{
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

