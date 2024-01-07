use crate::token::{Object, Token};

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Unary(UnaryExpr),
    Literal(LiteralExpr),
}

pub trait ExprVisitor<T> {
    fn visit_binary(&self, binary: &BinaryExpr) -> T;
    fn visit_grouping(&self, grouping: &GroupingExpr) -> T;
    fn visit_unary(&self, unary: &UnaryExpr) -> T;
    fn visit_literal(&self, literal: &LiteralExpr) -> T;
}

macro_rules! define_ast {
    ( $($name:ident, $visitor:ident : $($field:ident : $type:ty),*);* ) => {
        $(
            #[derive(Debug)]
            pub struct $name {
                $(
                    pub $field: $type,
                )*
            }

            impl $name {
                pub fn accept<T>(&self, visitor: &impl ExprVisitor<T>) -> T {
                    visitor.$visitor(self)
                }
            }
        )*
    };
}

define_ast!(BinaryExpr, visit_binary : left: Box<Expr>, operator: Token, right: Box<Expr>
    ;GroupingExpr, visit_grouping : expr: Box<Expr>
    ;UnaryExpr, visit_unary : operator: Token, right: Box<Expr>
    ;LiteralExpr, visit_literal : value: Object);
