use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextSpan {
    pub start: usize,
    pub end: usize,
    pub literal: String,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self {
            start,
            end,
            literal,
        }
    }

    pub fn combinar(mut spans: Vec<TextSpan>) -> TextSpan {
        if spans.is_empty() {
            println!("Não é possível combinar spans vazios");
        }
        spans.sort_by(|a, b| a.start.cmp(&b.start));

        let start = spans.first().unwrap().start;
        let end = spans.last().unwrap().end;

        TextSpan::new(
            start,
            end,
            spans.into_iter().map(|span| span.literal).collect(),
        )
    }

    pub fn comprimento(&self) -> usize {
        self.end - self.start
    }

    pub fn literal<'a>(&self, input: &'a str) -> &'a str {
        &input[self.start..self.end]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Numero(i64),
    Mais,
    Menos,
    Asterisco,
    Barra,
    FimDeArquivo,
    Erro,
    EspacoEmBranco,
    ParentesesEsquerdo,
    ParentesesDireito,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Numero(_) => write!(f, "Número"),
            TokenKind::Mais => write!(f, "+"),
            TokenKind::Menos => write!(f, "-"),
            TokenKind::Asterisco => write!(f, "*"),
            TokenKind::Barra => write!(f, "/"),
            TokenKind::FimDeArquivo => write!(f, "Fim de Arquivo"),
            TokenKind::Erro => write!(f, "Erro"),
            TokenKind::EspacoEmBranco => write!(f, "Espaço em Branco"),
            TokenKind::ParentesesEsquerdo => write!(f, "("),
            TokenKind::ParentesesDireito => write!(f, ")"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            current_pos: 0,
        }
    }

    pub fn proximo_token(&mut self) -> Option<Token> {
        if self.current_pos == self.input.len() {
            self.current_pos += 1;
            return Some(Token::new(
                TokenKind::FimDeArquivo,
                TextSpan::new(0, 0, '\0'.to_string()),
            ));
        }
    
        let start = self.current_pos;
        let c = self.caractere_atual();
    
        if let Some(c) = c {
            let kind = if c == '-' && self.caractere_seguinte().map_or(false, |next_c| next_c.is_digit(10)) {
                // Lidar com números negativos
                self.consumir(); // Consumir o sinal de menos
                let number = self.consumir_numero();
                TokenKind::Numero(-number) // Negar o número
            } else if Self::eh_numero(&c) {
                // Número regular
                let number: i64 = self.consumir_numero();
                TokenKind::Numero(number)
            } else if Self::eh_espaco_em_branco(&c) {
                self.consumir();
                TokenKind::EspacoEmBranco
            } else {
                self.consumir_pontuacao(c)
            };
    
            let end = self.current_pos;
            let literal = self.input[start..end].to_string();
            let span = TextSpan::new(start, end, literal);
    
            Some(Token::new(kind, span))
        } else {
            None
        }
    }
    
    fn caractere_seguinte(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos + 1)
    }

    fn consumir_pontuacao(&mut self, c: char) -> TokenKind {
        self.consumir();
        match c {
            '+' => TokenKind::Mais,
            '-' => TokenKind::Menos,
            '*' => TokenKind::Asterisco,
            '/' => TokenKind::Barra,
            '(' => TokenKind::ParentesesEsquerdo,
            ')' => TokenKind::ParentesesDireito,
            _ => TokenKind::Erro,
        }
    }

    fn eh_numero(c: &char) -> bool {
        c.is_digit(10)
    }

    fn eh_espaco_em_branco(c: &char) -> bool {
        c.is_whitespace()
    }

    fn caractere_atual(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    fn consumir(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            return None;
        }
        let c = self.caractere_atual();
        self.current_pos += 1;
        c
    }

    fn consumir_numero(&mut self) -> i64 {
        let mut number: i64 = 0;
        while let Some(c) = self.caractere_atual() {
            if c.is_digit(10) {
                self.consumir().unwrap();
                number = number * 10 + c.to_digit(10).unwrap() as i64;
            } else {
                break;
            }
        }
        number
    }
}
