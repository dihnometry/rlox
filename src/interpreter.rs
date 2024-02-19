use crate::error::Lox;
use crate::expr::{self, Expr, Visitor};
use crate::token::{Object, Token};
use crate::token_type::TokenType;


pub struct Interpreter;

struct RuntimeError {
    message: &'static str,
    token: Token,
}

impl RuntimeError {
    fn new(message: &'static str, token: Token) -> Self {
        RuntimeError {
            message,
            token
        }
    }
}

impl Visitor<Result<Object, RuntimeError>> for Interpreter {
    fn visit_binary(&self, binary: &expr::BinaryExpr) -> Result<Object, RuntimeError> {
        let left = self.evaluate(&binary.left)?;
        let right = self .evaluate(&binary.right)?;
        let op = binary.operator.clone();

        match binary.operator.ttype {
            TokenType::Minus => {
                match (left, right) {
                    (Object::Num(a), Object::Num(b)) => Ok(Object::Num(a + b)),
                    _ => Err(RuntimeError::new("Operands must be two numbers.", op)),
                }
            }
            TokenType::Slash => {
                match (left, right) {
                    (Object::Num(a), Object::Num(b)) => {
                        if b == 0.0 {
                            return Err(RuntimeError::new("Cannot divide by zero.", op))
                        }
                        Ok(Object::Num(a / b))
                    }
                    _ => Err(RuntimeError::new("Operands must be two numbers.", op)),
                }
            }
            TokenType::Star => {
                match (left, right) {
                    (Object::Num(a), Object::Num(b)) => Ok(Object::Num(a * b)),
                    _ => Err(RuntimeError::new("Operands must be two numbers.", op)),
                }
            }
            TokenType::Plus => {
                match (left, right) {
                    (Object::Num(a), Object::Num(b)) => Ok(Object::Num(a * b)),
                    (Object::Str(a), Object::Str(b)) => Ok(Object::Str(format!("{a}{b}"))),
                    _ => Err(RuntimeError::new("Operands must be two numbers or two strings.", op)),
                }
            }
            TokenType::Greater => {
                match (left, right) {
                    (Object::Num(a), Object::Num(b)) => Ok(Object::Boolean(a > b)),
                    _ => Err(RuntimeError::new("Operands must be two numbers.", op)),
                }
            }
            TokenType::GreaterEqual => {
                match (left, right) {
                    (Object::Num(a), Object::Num(b)) => Ok(Object::Boolean(a >= b)),
                    _ => Err(RuntimeError::new("Operands must be two numbers.", op)),
                }
            }
            TokenType::Less => {
                match (left, right) {
                    (Object::Num(a), Object::Num(b)) => Ok(Object::Boolean(a < b)),
                    _ => Err(RuntimeError::new("Operands must be two numbers.", op)),
                }
            }
            TokenType::LessEqual => {
                match (left, right) {
                    (Object::Num(a), Object::Num(b)) => Ok(Object::Boolean(a <= b)),
                    _ => Err(RuntimeError::new("Operands must be two numbers.", op)),
                }
            }
            TokenType::Equals => Ok(Object::Boolean(Self::is_equal(left, right))),
            TokenType::BangEqual => Ok(Object::Boolean(!Self::is_equal(left, right))),
            _ => todo!(),
        }
    }

    fn visit_grouping(&self, grouping: &expr::GroupingExpr) -> Result<Object, RuntimeError> {
        self.evaluate(&grouping.expr)
    }

    fn visit_unary(&self, unary: &expr::UnaryExpr) -> Result<Object, RuntimeError> {
        let right = self.evaluate(&unary.right)?;
        let op = unary.operator.clone();

        match unary.operator.ttype {
            TokenType::Minus => {
                match right {
                    Object::Num(num) => Ok(Object::Num(-num)),
                    _ => Err(RuntimeError::new("Operand must be a number.", op)),
                }
            }
            TokenType::Bang => {
                match right {
                    Object::Boolean(b) => Ok(Object::Boolean(!b)),
                    _ => Err(RuntimeError::new("Operand must be a boolean.", op)),
                }
            }
            _ => unreachable!(),
        }
    }

    fn visit_literal(&self, literal: &expr::LiteralExpr) -> Result<Object, RuntimeError> {
        Ok(literal.value.clone())
    }
}

impl Interpreter {
    pub fn interpret(&self, expr: Expr) {
        let result = Self::evaluate(&Interpreter, &expr);
        match result {
            Ok(value) => println!("{}", value.to_string()),
            Err(error) => Lox::runtime_error(&error.token, error.message)
        }
    }

    fn evaluate(&self, expr: &Expr) -> Result<Object, RuntimeError> {
        match expr {
            Expr::Binary(expr) => expr.accept(self),
            Expr::Grouping(expr) => expr.accept(self),
            Expr::Unary(expr) => expr.accept(self),
            Expr::Literal(expr) => expr.accept(self),
        }
    }

    fn is_equal(a: Object, b: Object) -> bool {
        match (a, b) {
            (Object::Num(a), Object::Num(b)) => a == b,
            (Object::Str(a), Object::Str(b)) => a == b,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::Nil, Object::Nil) => true,
            _ => false,
        }

    }
}
