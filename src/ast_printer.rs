use crate::expr::{self, Expr, Visitor};

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_binary(&self, binary: &expr::BinaryExpr) -> String {
        let expr_vec = &[&*binary.left, &*binary.right];
        AstPrinter::parenthesize(&binary.operator.lexeme, expr_vec, self)
    }

    fn visit_grouping(&self, grouping: &expr::GroupingExpr) -> String {
        AstPrinter::parenthesize("group", &[&*grouping.expr], self)
    }

    fn visit_unary(&self, unary: &expr::UnaryExpr) -> String {
        AstPrinter::parenthesize(
            &unary.operator.lexeme,
            &[&*unary.right],
            self,
        )
    }

    fn visit_literal(&self, literal: &expr::LiteralExpr) -> String {
        literal.value.to_string()
    }
}

impl AstPrinter {
    pub fn print(expr: &Expr, visitor: &impl Visitor<String>) -> String {
        AstPrinter::get_expression_string(expr, visitor)
    }

    pub fn parenthesize(
        name: &str,
        exprs: &[&Expr],
        visitor: &impl Visitor<String>,
    ) -> String {
        let mut string = format!("({name}");
        for expression in exprs {
            string.push(' ');

            let str = AstPrinter::get_expression_string(expression, visitor);

            string.push_str(&str);
        }
        string.push(')');

        string
    }

    fn get_expression_string(expr: &Expr, visitor: &impl Visitor<String>) -> String {
        match expr {
            Expr::Binary(binary) => binary.accept(visitor),
            Expr::Grouping(grouping) => grouping.accept(visitor),
            Expr::Unary(unary) => unary.accept(visitor),
            Expr::Literal(literal) => literal.accept(visitor),
        }
    }
}
