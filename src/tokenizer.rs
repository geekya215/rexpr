use std::error::Error;
use std::fmt::Display;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Space,
    Number(String),
    Operator(String),
    LParent,
    RParent,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Space => todo!(),
            Token::Number(n) => write!(f, "{}", n),
            Token::Operator(op) => write!(f, "{}", op),
            Token::LParent => f.write_str("("),
            Token::RParent => f.write_str(")"),
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
                Token::Space | Token::LParent | Token::RParent => self.col += 1,
                Token::Number(n) => self.col += n.len() as u32,
                Token::Operator(op) => self.col += op.len() as u32,
            }
            tokens.push(tok)
        }

        Ok(tokens)
    }

    fn next_token(&self, chars: &mut Peekable<Chars<'_>>) -> Result<Option<Token>, TokenizerError> {
        match chars.peek() {
            Some(&c) => match c {
                ' ' => self.consume(chars, Token::Space),
                '(' => self.consume(chars, Token::LParent),
                ')' => self.consume(chars, Token::RParent),
                '+' | '-' | '*' | '/' | '%' => self.consume(chars, Token::Operator(c.to_string())),
                '0'..='9' => {
                    let mut s = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ('0'..='9').contains(&ch) {
                            chars.next();
                            s.push(ch)
                        } else {
                            break;
                        }
                    }
                    Ok(Some(Token::Number(s)))
                }
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
            Token::Space,
            Token::Number(String::from("456")),
        ];

        assert_eq!(actual_tokens, expected_tokens)
    }

    #[test]
    fn tokenize_operator() {
        let operators = String::from("+ - * / %");
        let mut tokenizer = Tokenizer::new(&operators);
        let actual_tokens = tokenizer.tokenize().unwrap();

        let expected_tokens = vec![
            Token::Operator(String::from("+")),
            Token::Space,
            Token::Operator(String::from("-")),
            Token::Space,
            Token::Operator(String::from("*")),
            Token::Space,
            Token::Operator(String::from("/")),
            Token::Space,
            Token::Operator(String::from("%")),
        ];

        assert_eq!(actual_tokens, expected_tokens)
    }

    #[test]
    fn tokenize_with_parenthesis() {
        let arithmetic = String::from("(1 + 2) * 3");
        let mut tokenizer = Tokenizer::new(&arithmetic);
        let actual_tokens = tokenizer.tokenize().unwrap();

        let expected_tokens = vec![
            Token::LParent,
            Token::Number(String::from("1")),
            Token::Space,
            Token::Operator(String::from("+")),
            Token::Space,
            Token::Number(String::from("2")),
            Token::RParent,
            Token::Space,
            Token::Operator(String::from("*")),
            Token::Space,
            Token::Number(String::from("3")),
        ];

        assert_eq!(actual_tokens, expected_tokens)
    }
}
