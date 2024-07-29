mod ast;
use std::io;
use crate::ast::parser::{Parser};
use crate::ast::lexer::{Lexer, Token, TokenKind};

fn tokenize(lexer: &mut Lexer, tokens: &mut Vec<Token>) {
    while let Some(token) = lexer.next_token() {
        if token.kind != TokenKind::Whitespace {
            if token.kind == TokenKind::Eof {
                break;
            }
            if token.kind == TokenKind::Err {
                println!("Invalid token found");
                return; // Saímos da função se encontramos um token inválido
            }
            tokens.push(token);
        }
    }
}

fn handle_array_token() -> Vec<Token> {
    println!("Digite a expressão matemática: ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Erro ao ler a linha");

    let mut lexer = Lexer::new(&input);
    let mut tokens = Vec::new();

    tokenize(&mut lexer, &mut tokens);

    tokens
}

fn main() {
    let tokens = handle_array_token();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let _evaluated = ast.eval_step();
}
