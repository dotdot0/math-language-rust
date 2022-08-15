use crate::token::*;

#[derive(Debug)]
pub struct Expr{
   pub lhs: i32,
   pub rhs: i32,
   pub opr: String
}



pub fn parse(tokens: Vec<Token>) -> Result<Expr, String>{
    let token_type: Vec<TokenType> = vec![TokenType::Add, TokenType::Subtract, TokenType::Divide, TokenType::Multiply];
    let mut expr: Option<Expr> = None;
    for (i, token) in tokens.iter().enumerate(){
       if token_type.contains(&token.ttype){
           expr = Some(Expr{
               lhs: tokens[i - 1].parse_number(),
               rhs: tokens[i + 1].parse_number(),
               opr: token.value.to_string()
           })
       }
    }
    Ok(expr.unwrap())
}
