
use crate::token::Token;



pub fn parse(tokens: Vec<Token>){
    for token in tokens{
        println!("{:?}", token)
    }
}

