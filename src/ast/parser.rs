use std::collections::VecDeque;
use crate::ast::lexer::{Token, TokenKind};

#[derive(Debug, Clone)]
pub enum Node {
    Numero(i64),
    BinaryOp {
        op: TokenKind,
        left: Box<Node>,
        right: Box<Node>,
    },
}

impl Node {
    // Método para obter o valor de um nó de número

    pub fn avaliar(&self, arvore_str: &mut String) -> i64 {
        match self {
            Node::Numero(val) => *val,
            Node::BinaryOp { op, left, right } => {
                let left_val = left.avaliar(arvore_str);
                let right_val = right.avaliar(arvore_str);

                // Cria uma string representando a expressão atual
                let op_str = format!(
                    "({} {} {})",
                    left_val.to_string(),
                    match op {
                        TokenKind::Mais => "+",
                        TokenKind::Menos => "-",
                        TokenKind::Asterisco => "*",
                        TokenKind::Barra => "/",
                        _ => panic!("Operador inesperado"),
                    },
                    right_val.to_string()
                );

                let resultado = match op {
                    TokenKind::Mais => left_val + right_val,
                    TokenKind::Menos => left_val - right_val,
                    TokenKind::Asterisco => left_val * right_val,
                    TokenKind::Barra => left_val / right_val,
                    _ => panic!("Operador não suportado"),
                };


                // Verifica se a expressão está na string antes da substituição
                if arvore_str.contains(&op_str) {
                    // Substitui a expressão atual pela avaliação
                    *arvore_str = arvore_str.replace(&op_str, &resultado.to_string());

                    // Imprime a árvore atualizada
                } else {
                    // Adiciona mensagens de depuração detalhadas
                    println!("Erro: A expressão '{}' não foi encontrada na string da árvore '{}'", op_str, arvore_str);
                }
                println!("{}", arvore_str);
                resultado
            }
        }
    }

    pub fn para_string(&self) -> String {
        match self {
            Node::Numero(val) => val.to_string(),
            Node::BinaryOp { op, left, right } => {
                let left_str = left.para_string();
                let right_str = right.para_string();

                // Adiciona parênteses para garantir a ordem correta das operações
                format!("({} {} {})", left_str, op, right_str)
            }
        }
    }

}

#[derive(Debug)]
pub struct Ast {
    raiz: Option<Node>,
}

impl Ast {
    pub fn nova(raiz: Option<Node>) -> Self {
        Self { raiz }
    }

    pub fn raiz(&self) -> Option<&Node> {
        self.raiz.as_ref()
    }

    pub fn eval_step(&self) -> i64 {
        // Cria uma string mutável para a árvore de expressão
        let mut arvore_str = self.raiz.as_ref().map_or("".to_string(), |node| node.para_string());

        // Avalia a expressão e imprime passo a passo
        self.raiz.as_ref().map_or(0, |node| node.avaliar(&mut arvore_str))
    }

}

pub struct Parser {
    tokens: Vec<Token>,
    atual: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            atual: 0,
        }
    }

    fn proximo_token(&mut self) -> Option<&Token> {
        if self.atual < self.tokens.len() {
            let token = &self.tokens[self.atual];
            self.atual += 1;
            Some(token)
        } else {
            None
        }
    }

    fn precedencia(op: &TokenKind) -> u8 {
        match op {
            TokenKind::Mais | TokenKind::Menos => 1,
            TokenKind::Asterisco | TokenKind::Barra => 2,
            _ => 0,
        }
    }

    pub fn parse(&mut self) -> Ast {
        let mut saida = VecDeque::new();
        let mut operadores = Vec::new();

        while let Some(token) = self.proximo_token() {
            match &token.kind {
                TokenKind::Numero(val) => {
                    saida.push_back(Node::Numero(*val));
                }
                TokenKind::Mais | TokenKind::Menos | TokenKind::Asterisco | TokenKind::Barra => {
                    while let Some(op) = operadores.last() {
                        if Self::precedencia(op) >= Self::precedencia(&token.kind) {
                            let op = operadores.pop().unwrap();
                            let direita = saida.pop_back().unwrap();
                            let esquerda = saida.pop_back().unwrap();
                            saida.push_back(Node::BinaryOp {
                                op,
                                left: Box::new(esquerda),
                                right: Box::new(direita),
                            });
                        } else {
                            break;
                        }
                    }
                    operadores.push(token.kind.clone());
                }
                TokenKind::ParentesesEsquerdo => {
                    operadores.push(token.kind.clone());
                }
                TokenKind::ParentesesDireito => {
                    while let Some(op) = operadores.pop() {
                        if let TokenKind::ParentesesEsquerdo = op {
                            break;
                        }
                        let direita = saida.pop_back().unwrap();
                        let esquerda = saida.pop_back().unwrap();
                        saida.push_back(Node::BinaryOp {
                            op,
                            left: Box::new(esquerda),
                            right: Box::new(direita),
                        });
                    }
                }
                _ => {}
            }
        }

        while let Some(op) = operadores.pop() {
            let direita = saida.pop_back().unwrap();
            let esquerda = saida.pop_back().unwrap();
            saida.push_back(Node::BinaryOp {
                op,
                left: Box::new(esquerda),
                right: Box::new(direita),
            });
        }

        Ast::nova(saida.pop_back())
    }
}
