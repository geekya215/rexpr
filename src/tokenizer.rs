use std::error::Error;
use std::fmt::Display;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Space,
    Number(String),
    Plus,
    Minus,
    Mul,
    Div,
    LParen,
    RParen,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Space => f.write_str(" "),
            Token::Number(n) => write!(f, "{}", n),
            Token::Plus => f.write_str("+"),
            Token::Minus => f.write_str("-"),
            Token::Mul => f.write_str("*"),
            Token::Div => f.write_str("/"),
            Token::LParen => f.write_str("("),
            Token::RParen => f.write_str(")"),
        }
    }
}

#[derive(Debug)]
pub struct TokenizerError {
    pub message: String,
    pub line: u32,
    pub col: u32,
}

impl Display for TokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at Line: {}, Column {}",
            self.message, self.line, self.col
        )
    }
}

impl Error for TokenizerError {}

pub struct Tokenizer<'a> {
    pub text: &'a str,
    pub line: u32,
    pub col: u32,
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Self {
        Tokenizer {
            text,
            line: 1,
            col: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, TokenizerError> {
        let mut peekable = self.text.chars().peekable();
        let mut tokens: Vec<Token> = vec![];

        while let Some(tok) = self.next_token(&mut peekable)? {
            match &tok {
                Token::Space | Token::LParen | Token::RParen => self.col += 1,
                Token::Number(n) => self.col += n.len() as u32,
                Token::Plus | Token::Minus | Token::Mul | Token::Div => self.col += 1,
            }
            if tok == Token::Space {
                continue;
            }
            tokens.push(tok)
        }

        Ok(tokens)
    }

    fn next_token(&self, chars: &mut Peekable<Chars<'_>>) -> Result<Option<Token>, TokenizerError> {
        match chars.peek() {
            Some(&c) => match c {
                ' ' => self.consume(chars, Token::Space),
                '(' => self.consume(chars, Token::LParen),
                ')' => self.consume(chars, Token::RParen),
                '+' => self.consume(chars, Token::Plus),
                '-' => self.consume(chars, Token::Minus),
                '*' => self.consume(chars, Token::Mul),
                '/' => self.consume(chars, Token::Div),
                '0'..='9' => Ok(Some(Token::Number(
                    self.take_while(chars, |ch| matches!(ch, '0'..='9')),
                ))),
                _ => Err(TokenizerError {
                    message: "Unknow symbol".to_string(),
                    line: self.line,
                    col: self.col,
                }),
            },
            None => Ok(None),
        }
    }

    fn consume(
        &self,
        chars: &mut Peekable<Chars<'_>>,
        token: Token,
    ) -> Result<Option<Token>, TokenizerError> {
        chars.next();
        Ok(Some(token))
    }

    fn take_while(
        &self,
        chars: &mut Peekable<Chars<'_>>,
        mut predicate: impl FnMut(char) -> bool,
    ) -> String {
        let mut s = String::new();
        while let Some(&ch) = chars.peek() {
            if predicate(ch) {
                chars.next();
                s.push(ch);
            } else {
                break;
            }
        }
        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenize_number() {
        let nubmer = String::from("123 456");
        let mut tokenizer = Tokenizer::new(&nubmer);
        let actual_tokens = tokenizer.tokenize().unwrap();

        let expected_tokens = vec![
            Token::Number(String::from("123")),
            Token::Number(String::from("456")),
        ];

        assert_eq!(actual_tokens, expected_tokens)
    }

    #[test]
    fn tokenize_operator() {
        let operators = String::from("+ - * /");
        let mut tokenizer = Tokenizer::new(&operators);
        let actual_tokens = tokenizer.tokenize().unwrap();

        let expected_tokens = vec![Token::Plus, Token::Minus, Token::Mul, Token::Div];

        assert_eq!(actual_tokens, expected_tokens)
    }

    #[test]
    fn tokenize_with_parenthesis() {
        let arithmetic = String::from("(1 + 2) * 3");
        let mut tokenizer = Tokenizer::new(&arithmetic);
        let actual_tokens = tokenizer.tokenize().unwrap();

        let expected_tokens = vec![
            Token::LParen,
            Token::Number(String::from("1")),
            Token::Plus,
            Token::Number(String::from("2")),
            Token::RParen,
            Token::Mul,
            Token::Number(String::from("3")),
        ];

        assert_eq!(actual_tokens, expected_tokens)
    }
}
