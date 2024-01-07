use std::ops::Deref;

use crate::{expr::{self, ExprVisitor, Expr}, token_type::TokenType, token::{Token, Object}};

pub struct AstPrinter;

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary(&self, binary: &expr::BinaryExpr) -> String {
        let expr_vec = vec![binary.left.deref(), binary.right.deref()];
        AstPrinter::parenthesize(binary.operator.lexeme.clone(), expr_vec, self)
    }

    fn visit_grouping(&self, grouping: &expr::GroupingExpr) -> String {
        AstPrinter::parenthesize(String::from("group"), vec![grouping.expr.deref()], self)
    }

    fn visit_unary(&self, unary: &expr::UnaryExpr) -> String {
        AstPrinter::parenthesize(unary.operator.lexeme.clone(), vec![unary.right.deref()], self)
    }

    fn visit_literal(&self, literal: &expr::LiteralExpr) -> String {
        literal.value.to_string()
    }
}

impl AstPrinter {
    pub fn print(expr: &Expr, visitor: &impl ExprVisitor<String>) -> String {
        AstPrinter::get_expression_string(expr, visitor)
    }

    pub fn parenthesize(name: String, exprs: Vec<&Expr>, visitor: &impl ExprVisitor<String>) -> String {
        let mut string = format!("({}", name);
        for expression in exprs.iter() {
            string.push(' ');

            let str = AstPrinter::get_expression_string(expression, visitor);

            string.push_str(&str);
        }
        string.push(')');

        string
    }

    fn get_expression_string(expr: &Expr, visitor: &impl ExprVisitor<String>) -> String {
        match expr {
            Expr::Binary(binary) => binary.accept(visitor),
            Expr::Grouping(grouping) => grouping.accept(visitor),
            Expr::Unary(unary) => unary.accept(visitor),
            Expr::Literal(literal) => literal.accept(visitor),
        }
    }
}
