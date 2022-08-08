
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub enum AstType{
    VariableAssign,
    Operation
}

#[derive(Debug)]
pub struct Assigment{
    variable: String, 
    value: String
}

#[derive(Debug)]
pub struct Ast<T>{
    ty:AstType,
    value: T
}

pub fn parse(tokens: Vec<Token>) -> Vec<Ast<Assigment>>{
    let mut asts = Vec::new();
    let mut current_node: Option<Ast<Assigment>> = None;
    for (i, token) in tokens.iter().enumerate(){
        if token.ttype == TokenType::Assign{
            current_node = Some(Ast{
                ty: AstType::VariableAssign,
                value: Assigment{
                    variable: tokens[i - 1].value.to_string(),
                    value: tokens[i + 1].value.to_string()
                }
            });
            asts.push(current_node.unwrap())
        }
    }
    asts
}

