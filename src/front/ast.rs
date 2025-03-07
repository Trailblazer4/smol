//! The abstract syntax tree.

use derive_more::derive::Display;

use crate::common::Id;

#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Stmt {
    Assign(Id, Expr),
    Print(Expr),
    Read(Id),
    If {
        guard: Expr,
        tt: Vec<Stmt>,
        ff: Vec<Stmt>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Var(Id),
    Const(i64),
    BinOp {
        op: BOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Negate(Box<Expr>),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Display)]
pub enum BOp {
    #[display("mul")]
    Mul,
    #[display("div")]
    Div,
    #[display("add")]
    Add,
    #[display("sub")]
    Sub,
    #[display("lt")]
    Lt,
}
