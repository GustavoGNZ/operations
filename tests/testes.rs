use operations::ast::lexer::{Lexer, TokenKind, Token};
use operations::ast::parser::{Parser};

fn tokenize(lexer: &mut Lexer, tokens: &mut Vec<Token>) {
    while let Some(token) = lexer.proximo_token() {
        if token.kind != TokenKind::EspacoEmBranco {
            if token.kind == TokenKind::FimDeArquivo {
                break;
            }
            if token.kind == TokenKind::Erro {
                println!("Token invalido encontrado");
                return; // Saímos da função se encontramos um token inválido
            }
            tokens.push(token);
        }
    }
}
#[test]
fn test_case_1() {
    let mut lexer = Lexer::new("1 + 3");
    let mut tokens = Vec::new();

    tokenize(&mut lexer, &mut tokens);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let result = ast.eval_step();
    assert_eq!(result, 4); // O Resultado esperado eh 4
}

#[test]
fn test_case_2() {
    let mut lexer = Lexer::new("1 + 2 * 3");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, 7);
}

#[test]
fn test_case_3() {
    let mut lexer = Lexer::new("4 / 2 + 7");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, 9);
}

#[test]
fn test_case_4() {
    let mut lexer = Lexer::new("1 + 2 + 3 * 4");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, 15);
}

#[test]
fn test_case_5() {
    let mut lexer = Lexer::new("(1 + 2 + 3) * 4");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, 24);
}

#[test]
fn test_case_6() {
    let mut lexer = Lexer::new("(10 / 3 + 23) * (1 - 4)");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, -78);
}

#[test]
fn test_case_7() {
    let mut lexer = Lexer::new("((1 + 3) * 8 + 1) / 3");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, 11);
}

#[test]
fn test_case_8() {
    let mut lexer = Lexer::new("58 - -8 * (58 + 31) - -14");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, 784);
}

#[test]
fn test_case_9() {
    let mut lexer = Lexer::new("-71 * (-76 * 91 * (10 - 5 - -82) - -79)");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, 42714523);
}

#[test]
fn test_case_10() {
    let mut lexer = Lexer::new("10 * 20 + 3 * 7 + 2 * 3 + 10 / 3 * 4");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, 239);
}

#[test]
fn test_case_11() {
    let mut lexer = Lexer::new("(-13 - -73) * (44 - -78 - 77 + 42 - -32)");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, 7140);
}

#[test]
fn test_case_12() {
    let mut lexer = Lexer::new("-29 * 49 + 47 - 29 + 74 - -85 - -27 + 4 - 28");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, -1241);
}

#[test]
fn test_case_13() {
    let mut lexer = Lexer::new("-74 - -14 + 42 - -4 + -78 + -50 * -35 * -81 + -41");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, -141883);
}

#[test]
fn test_case_14() {
    let mut lexer = Lexer::new("80 * -18 * (85 * (-46 + -71) - 12 + 26 - 59) + 84");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, 14385684);
}

#[test]
fn test_case_15() {
    let mut lexer = Lexer::new("25 + 38 + 88 + (-6 - -73) * (-83 + (53 + 97) * 14)");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, 135290);
}

#[test]
fn test_case_16() {
    let mut lexer = Lexer::new("(84 - 90) * (-8 - 75 + -83 * (56 - -77) + 4 + -94)");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, 67272);
}

#[test]
fn test_case_17() {
    let mut lexer = Lexer::new("(54 - -8 - -35 + -68 - -90) * -39 + -43 + -91 * -30");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, -1954);
}

#[test]
fn test_case_18() {
    let mut lexer = Lexer::new("-13 - -74 + (66 + -57) * -93 * -9 * 77 + 79 - 66 + -53");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, 580062);
}

#[test]
fn test_case_19() {
    let mut lexer = Lexer::new("(-72 - 50 * -74 + -45) * 92 * 21 * 5 * (-13 - 66 - 18)");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, -3357342660);
}

#[test]
fn test_case_20() {
    let mut lexer = Lexer::new("-7 - -37 * (90 + 70) - 30 - -44 + -32 - 56 - -48 - -78");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, 5965);
}

#[test]
fn test_case_21() {
    let mut lexer = Lexer::new("65 * -83 - -3 + -20 + 24 - 85 * (-24 + -32) * (61 - 20)");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, 189772);
}

#[test]
fn test_case_22() {
    let mut lexer = Lexer::new("55 * 48 * -44 - -32 + 1 * -80 * -94 - 74 * -53 + -30 + -61");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, -104777);
}

#[test]
fn test_case_23() {
    let mut lexer = Lexer::new("(-82 * (25 + 62 + 3) - -72 + -65 * -32 * (77 + 12) - -95 + 51)");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, 177958);
}

#[test]
fn test_case_24() {
    let mut lexer = Lexer::new("(2 - 65 - (-24 + -97) * -5 * -61) * (-41 + 85 * 9 * -92 * (75 - 18))");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, -147799088242);
}

#[test]
fn test_case_25() {
    let mut lexer = Lexer::new("-20 + -51 + 20 + -68 * -11 + -35 * -14 - 95 - 32 + -52 * -23 - -90 * -42");
    let mut tokens = Vec::new();
    tokenize(&mut lexer, &mut tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = ast.eval_step();
    assert_eq!(result, -1524);
}
