use crate::parser::*;

pub struct Eval {}

impl Eval {
    pub fn new() -> Self {
        Eval {}
    }

    pub fn eval(&self, node: &Node) -> i32 {
        match node {
            Node::Number(n) => *n,
            Node::UnaryExpr { op, child } => {
                let child = self.eval(&child);
                match op {
                    UnaryOperator::Neg => -child,
                }
            }
            Node::BinaryExpr { op, lhs, rhs } => {
                let left_result = self.eval(&lhs);
                let right_result = self.eval(&rhs);

                match op {
                    BinaryOperator::Plus => left_result + right_result,
                    BinaryOperator::Minus => left_result - right_result,
                    BinaryOperator::Mul => left_result * right_result,
                    BinaryOperator::Div => left_result / right_result,
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tokenizer::Tokenizer;

    use super::*;

    #[test]
    fn number() {
        let mut tokenizer = Tokenizer::new("1");
        let tokens = tokenizer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        let eval = Eval::new();
        let result = eval.eval(&expr);
        assert_eq!(1, result)
    }

    #[test]
    fn negtive_number() {
        let mut tokenizer = Tokenizer::new("-1");
        let tokens = tokenizer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        let eval = Eval::new();
        let result = eval.eval(&expr);
        assert_eq!(-1, result)
    }

    #[test]
    fn plus() {
        let mut tokenizer = Tokenizer::new("1 + 2");
        let tokens = tokenizer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        let eval = Eval::new();
        let result = eval.eval(&expr);
        assert_eq!(3, result)
    }

    #[test]
    fn minus() {
        let mut tokenizer = Tokenizer::new("1 - 2");
        let tokens = tokenizer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        let eval = Eval::new();
        let result = eval.eval(&expr);
        assert_eq!(-1, result)
    }

    #[test]
    fn mul() {
        let mut tokenizer = Tokenizer::new("1 * 2");
        let tokens = tokenizer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        let eval = Eval::new();
        let result = eval.eval(&expr);
        assert_eq!(2, result)
    }

    #[test]
    fn div() {
        let mut tokenizer = Tokenizer::new("1 / 2");
        let tokens = tokenizer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        let eval = Eval::new();
        let result = eval.eval(&expr);
        assert_eq!(0, result)
    }

    #[test]
    fn normal_expr() {
        let mut tokenizer = Tokenizer::new("1 + 2 * 3");
        let tokens = tokenizer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        let eval = Eval::new();
        let result = eval.eval(&expr);
        assert_eq!(7, result)
    }

    #[test]
    fn parenthesis_expr() {
        let mut tokenizer = Tokenizer::new("(1 + 2) * 3");
        let tokens = tokenizer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        let eval = Eval::new();
        let result = eval.eval(&expr);
        assert_eq!(9, result)
    }

    #[test]
    fn negetive_expr() {
        let mut tokenizer = Tokenizer::new("-(1 + 2) * 3");
        let tokens = tokenizer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        let eval = Eval::new();
        let result = eval.eval(&expr);
        assert_eq!(-9, result)
    }

    #[test]
    fn negetive_number_in_expr() {
        let mut tokenizer = Tokenizer::new("1 + -2 * 3");
        let tokens = tokenizer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        let eval = Eval::new();
        let result = eval.eval(&expr);
        assert_eq!(-5, result)
    }
}
