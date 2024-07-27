mod ast;
use std::io;

fn handle_array_token() -> Vec<String> {

    println!("Digite a expressão matemática: ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Erro ao ler a linha");

    let mut lexer = ast::lexer::Lexer::new(&input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token() {
        if token.kind != ast::lexer::TokenKind::Whitespace{
            if token.kind == ast::lexer::TokenKind::Eof {
                break;
            }
            if token.kind == ast::lexer::TokenKind::Err {
                panic!("Invalid token found");
            }
            tokens.push(token.span.literal);
        }
    }
    return tokens;

}

fn main () {

    let _operation_array = handle_array_token();


}