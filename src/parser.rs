use crate::tokenizer::*;
use std::error::Error;
use std::fmt::Display;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOperator {
    Neg,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Number(i32),
    BinaryExpr {
        op: BinaryOperator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    UnaryExpr {
        op: UnaryOperator,
        child: Box<Node>,
    },
}

#[derive(Debug)]
pub struct ParserError {
    message: String,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParserError: {}", self.message)
    }
}

impl Error for ParserError {}

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens }
    }

    ///
    /// expr -> term ord_1_op expr | term
    /// term -> factor ord_2_op term | factor
    /// factor -> number | neg factor | lparen expr rparen
    /// neg -> -
    /// ord_1_op -> + | -
    /// ord_2_op -> * | /
    /// lparen -> (
    /// rparen -> )
    ///
    pub fn parse(&self) -> Result<Node, ParserError> {
        self.parse_expr(&mut self.tokens.iter().peekable())
    }

    fn parse_expr(&self, tokens: &mut Peekable<Iter<Token>>) -> Result<Node, ParserError> {
        let term = self.parse_term(tokens)?;
        let bop = match tokens.peek() {
            Some(op) => match op {
                Token::Plus => {
                    tokens.next();
                    BinaryOperator::Plus
                }
                Token::Minus => {
                    tokens.next();
                    BinaryOperator::Minus
                }
                _ => return Ok(term),
            },
            None => return Ok(term),
        };
        let expr = self.parse_expr(tokens)?;
        Ok(Node::BinaryExpr {
            op: bop,
            lhs: Box::new(term),
            rhs: Box::new(expr),
        })
    }

    fn parse_term(&self, tokens: &mut Peekable<Iter<Token>>) -> Result<Node, ParserError> {
        let factor = self.parse_factor(tokens)?;
        let bop = match tokens.peek() {
            Some(op) => match op {
                Token::Mul => {
                    tokens.next();
                    BinaryOperator::Mul
                }
                Token::Div => {
                    tokens.next();
                    BinaryOperator::Div
                }
                _ => return Ok(factor),
            },
            None => return Ok(factor),
        };
        let term = self.parse_term(tokens)?;
        Ok(Node::BinaryExpr {
            op: bop,
            lhs: Box::new(factor),
            rhs: Box::new(term),
        })
    }

    fn parse_factor(&self, tokens: &mut Peekable<Iter<Token>>) -> Result<Node, ParserError> {
        match tokens.peek() {
            Some(&factor) => match factor {
                Token::Number(n) => {
                    tokens.next();
                    Ok(Node::Number(n.parse::<i32>().unwrap()))
                }
                Token::Minus => {
                    tokens.next();
                    let factor = self.parse_factor(tokens)?;
                    Ok(Node::UnaryExpr {
                        op: UnaryOperator::Neg,
                        child: Box::new(factor),
                    })
                }
                Token::LParen => {
                    tokens.next();
                    let expr = self.parse_expr(tokens)?;
                    self.skip(|t| *t == Token::RParen, tokens)?;
                    Ok(expr)
                }
                other => Err(ParserError {
                    message: format!("unexpected token {}", other),
                }),
            },
            None => Err(ParserError {
                message: format!("expected factor"),
            }),
        }
    }

    fn skip(
        &self,
        mut predicate: impl FnMut(&Token) -> bool,
        tokens: &mut Peekable<Iter<Token>>,
    ) -> Result<(), ParserError> {
        match tokens.peek() {
            Some(&token) if predicate(token) => {
                tokens.next();
                Ok(())
            }
            _ => Err(ParserError {
                message: format!("unknow token"),
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_number() {
        let mut tokenizer = Tokenizer::new("1");
        let tokens = tokenizer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        assert_eq!(expr, Node::Number(1))
    }

    #[test]
    fn negtive_number() {
        let mut tokenizer = Tokenizer::new("-1");
        let tokens = tokenizer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        assert_eq!(
            expr,
            Node::UnaryExpr {
                op: UnaryOperator::Neg,
                child: Box::new(Node::Number(1))
            }
        )
    }

    #[test]
    fn negtive_expr() {
        let mut tokenizer = Tokenizer::new("-(1 + 2)");
        let tokens = tokenizer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        assert_eq!(
            expr,
            Node::UnaryExpr {
                op: UnaryOperator::Neg,
                child: Box::new(Node::BinaryExpr {
                    op: BinaryOperator::Plus,
                    lhs: Box::new(Node::Number(1)),
                    rhs: Box::new(Node::Number(2))
                })
            }
        )
    }
    #[test]
    fn normal_prior() {
        let mut tokenizer = Tokenizer::new("1 + 2 * 3");
        let tokens = tokenizer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        assert_eq!(
            expr,
            Node::BinaryExpr {
                op: BinaryOperator::Plus,
                lhs: Box::new(Node::Number(1)),
                rhs: Box::new(Node::BinaryExpr {
                    op: BinaryOperator::Mul,
                    lhs: Box::new(Node::Number(2)),
                    rhs: Box::new(Node::Number(3))
                })
            }
        )
    }

    #[test]
    fn parenthesis_prior() {
        let mut tokenizer = Tokenizer::new("(1 + 2) * 3");
        let tokens = tokenizer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        assert_eq!(
            expr,
            Node::BinaryExpr {
                op: BinaryOperator::Mul,
                lhs: Box::new(Node::BinaryExpr {
                    op: BinaryOperator::Plus,
                    lhs: Box::new(Node::Number(1)),
                    rhs: Box::new(Node::Number(2))
                }),
                rhs: Box::new(Node::Number(3)),
            }
        )
    }
}
