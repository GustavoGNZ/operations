mod ast;
use std::io;
use crate::ast::parser::{Parser};
use crate::ast::lexer::{Lexer, Token, TokenKind};

fn tokenize(lexer: &mut Lexer, tokens: &mut Vec<Token>) -> bool {
    while let Some(token) = lexer.proximo_token() {
        if token.kind != TokenKind::EspacoEmBranco {
            if token.kind == TokenKind::FimDeArquivo {
                break;
            }
            if token.kind == TokenKind::Erro {
                println!("Token inválido encontrado");
                return false; // Indica que encontramos um token inválido
            }
            tokens.push(token);
        }
    }
    true // Todos os tokens são válidos
}

fn handle_array_token() -> Vec<Token> {
    loop {
        println!("Digite a expressão matemática: ");
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Erro ao ler a linha");

        let mut lexer = Lexer::new(&input);
        let mut tokens = Vec::new();

        if tokenize(&mut lexer, &mut tokens) {
            return tokens; // Retorna apenas se os tokens forem válidos
        } else {
            println!("Expressão inválida. Por favor, tente novamente.");
        }
    }
}

fn main() {
    let tokens = handle_array_token();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let _evaluated = ast.eval_step();
}
