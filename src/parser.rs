use crate::token::*;

#[derive(Debug)]
pub enum Ast{
    Expr(Expr),
    Assigment(Assigment)
}

#[derive(Debug)]
pub struct Expr{
   pub lhs: i32,
   pub rhs: i32,
   pub opr: String
}

#[derive(Debug)]
pub struct Assigment{
    variable: String,
    value: i32
}



pub fn parse(tokens: Vec<Token>) -> Result<Ast, String>{
    let token_type: Vec<TokenType> = vec![TokenType::Add, TokenType::Subtract, TokenType::Divide, TokenType::Multiply, TokenType::GreaterThan, TokenType::LessThan];
    let mut expr: Option<Ast> = None;
    for (i, token) in tokens.iter().enumerate(){
       if token_type.contains(&token.ttype){
           expr = Some(Ast::Expr(Expr{
               lhs: tokens[i - 1].parse_number(),
               rhs: tokens[i + 1].parse_number(),
               opr: token.value.to_string()
           }))
       }
       else if token.ttype == TokenType::Assign{
           expr = Some(Ast::Assigment(Assigment{
               variable: tokens[i - 1].value.to_string(),
               value: tokens[i + 1].parse_number()
           }))
       }
    }
    Ok(expr.unwrap())
}
