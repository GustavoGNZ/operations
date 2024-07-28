use std::collections::VecDeque;
use crate::ast::lexer::{Token, TokenKind};

#[derive(Debug, Clone)]
pub enum Node {
    Number(i64),
    BinaryOp {
        op: TokenKind,
        left: Box<Node>,
        right: Box<Node>,
    },
}

impl Node {
    // Método para obter o valor de um nó de número

    pub fn evaluate(&self, tree_str: &mut String) -> i64 {
        match self {
            Node::Number(val) => *val,
            Node::BinaryOp { op, left, right } => {
                let left_val = left.evaluate(tree_str);
                let right_val = right.evaluate(tree_str);

                // Cria uma string representando a expressão atual
                let op_str = format!(
                    "({} {} {})",
                    left_val.to_string(),
                    match op {
                        TokenKind::Plus => "+",
                        TokenKind::Minus => "-",
                        TokenKind::Asterisk => "*",
                        TokenKind::Slash => "/",
                        _ => panic!("Unexpected operator"),
                    },
                    right_val.to_string()
                );

                let result = match op {
                    TokenKind::Plus => left_val + right_val,
                    TokenKind::Minus => left_val - right_val,
                    TokenKind::Asterisk => left_val * right_val,
                    TokenKind::Slash => left_val / right_val,
                    _ => panic!("Unsupported operator"),
                };


                // Verifica se a expressão está na string antes da substituição
                if tree_str.contains(&op_str) {
                    // Substitui a expressão atual pela avaliação
                    *tree_str = tree_str.replace(&op_str, &result.to_string());

                    // Imprime a árvore atualizada
                } else {
                    // Adiciona mensagens de depuração detalhadas
                    println!("Error: The expression '{}' was not found in the tree string '{}'", op_str, tree_str);
                }
                println!("{}", tree_str);
                result
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Node::Number(val) => val.to_string(),
            Node::BinaryOp { op, left, right } => {
                let left_str = left.to_string();
                let right_str = right.to_string();

                // Adiciona parênteses para garantir a ordem correta das operações
                format!("({} {} {})", left_str, op, right_str)
            }
        }
    }

}

#[derive(Debug)]
pub struct Ast {
    root: Option<Node>,
}

impl Ast {
    pub fn new(root: Option<Node>) -> Self {
        Self { root }
    }

    pub fn root(&self) -> Option<&Node> {
        self.root.as_ref()
    }

    pub fn eval_step(&self) -> i64 {
        // Cria uma string mutável para a árvore de expressão
        let mut tree_str = self.root.as_ref().map_or("".to_string(), |node| node.to_string());

        // Avalia a expressão e imprime passo a passo
        self.root.as_ref().map_or(0, |node| node.evaluate(&mut tree_str))
    }

}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    fn next_token(&mut self) -> Option<&Token> {
        if self.current < self.tokens.len() {
            let token = &self.tokens[self.current];
            self.current += 1;
            Some(token)
        } else {
            None
        }
    }

    fn precedence(op: &TokenKind) -> u8 {
        match op {
            TokenKind::Plus | TokenKind::Minus => 1,
            TokenKind::Asterisk | TokenKind::Slash => 2,
            _ => 0,
        }
    }

    pub fn parse(&mut self) -> Ast {
        let mut output = VecDeque::new();
        let mut operators = Vec::new();

        while let Some(token) = self.next_token() {
            match &token.kind {
                TokenKind::Number(val) => {
                    output.push_back(Node::Number(*val));
                }
                TokenKind::Plus | TokenKind::Minus | TokenKind::Asterisk | TokenKind::Slash => {
                    while let Some(op) = operators.last() {
                        if Self::precedence(op) >= Self::precedence(&token.kind) {
                            let op = operators.pop().unwrap();
                            let right = output.pop_back().unwrap();
                            let left = output.pop_back().unwrap();
                            output.push_back(Node::BinaryOp {
                                op,
                                left: Box::new(left),
                                right: Box::new(right),
                            });
                        } else {
                            break;
                        }
                    }
                    operators.push(token.kind.clone());
                }
                TokenKind::LeftParen => {
                    operators.push(token.kind.clone());
                }
                TokenKind::RightParen => {
                    while let Some(op) = operators.pop() {
                        if let TokenKind::LeftParen = op {
                            break;
                        }
                        let right = output.pop_back().unwrap();
                        let left = output.pop_back().unwrap();
                        output.push_back(Node::BinaryOp {
                            op,
                            left: Box::new(left),
                            right: Box::new(right),
                        });
                    }
                }
                _ => {}
            }
        }

        while let Some(op) = operators.pop() {
            let right = output.pop_back().unwrap();
            let left = output.pop_back().unwrap();
            output.push_back(Node::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            });
        }

        Ast::new(output.pop_back())
    }
}