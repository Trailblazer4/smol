//! Lowering

use super::ast;
use super::BOp;
use ast::Stmt;
use ast::Expr;
use crate::{
    common::{id, Id},
    middle::tir,
};
use std::collections::{BTreeMap as Map, BTreeSet as Set};
use tir::{Block, Instruction, Terminator};
use TvEntry::*;

pub fn lower(program: ast::Program) -> tir::Program {
    let lower = Lower::new();
    lower.lower_program(program)
}

// Entries in the translation vector
#[derive(Debug, Clone)]
enum TvEntry {
    // A basic block label
    Label(Id),
    // An inner (non-terminating) instruction
    Inner(Instruction),
    // A terminal instruction
    Term(Terminator),
}

impl TvEntry {
    fn get_inner(self) -> Option<Instruction> {
        if let Inner(i) = self {
            Some(i)
        } else {
            None
        }
    }
}

// Lowering data
struct Lower {
    decl: Set<Id>,
    // translation vector
    tv: Vec<TvEntry>,
    // for creating fresh locals
    fresh_ctr: i64,
    // for creating fresh block labels
    bb_ctr: i64,
}

impl Lower {
    fn new() -> Self {
        Lower {
            decl: Set::new(),
            tv: vec![],
            fresh_ctr: 0,
            bb_ctr: 0,
        }
    }

    // add given variable to declared variables
    fn add_decl(&mut self, var: Id) {
        self.decl.insert(var);
    }

    fn lower_program(mut self, program: ast::Program) -> tir::Program {
        self.tv.push(Label(id("entry")));

        for stmt in program.stmts {
            self.lower_stmt(stmt);
        }
        // Close the last basic block
        self.tv.push(Term(Terminator::Exit));

        tir::Program {
            decl: self.decl,
            block: construct_cfg(self.tv),
        }
    }

    fn lower_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Assign(dst, e) => {
                self.add_decl(dst);
                let src = self.lower_expr(e);
                self.tv.push(Inner(Instruction::Copy { dst, src }));
            }
            Stmt::Print(e) => {
                let x = self.lower_expr(e);
                self.tv.push(Inner(Instruction::Print(x)));
            }
            Stmt::Read(x) => {
                self.add_decl(x);
                self.tv.push(Inner(Instruction::Read(x)));
            }
            Stmt::If { guard, tt, ff } => {
                let lbl_tt = self.mk_label();
                let lbl_ff = self.mk_label();
                let lbl_join = self.mk_label();
                let guard = self.lower_expr(guard);
                self.tv.push(Term(Terminator::Branch { guard, tt: lbl_tt, ff: lbl_ff }));

                self.tv.push(Label(lbl_tt));
                for stmt in tt {
                    self.lower_stmt(stmt);
                }
                self.tv.push(Term(Terminator::Jump(lbl_join)));
                self.tv.push(Label(lbl_ff));
                for stmt in ff {
                    self.lower_stmt(stmt);
                }
                self.tv.push(Term(Terminator::Jump(lbl_join)));
                self.tv.push(Label(lbl_join));
            },
        }
    }

    fn lower_expr(&mut self, e: Expr) -> Id {
        match e {
            Expr::Var(x) => {
                self.add_decl(x);
                x
            }
            Expr::Const(n) => {
                // this is not as good as the IR generation I covered.
                let dst = self.mk_var("_const");
                self.tv.push(Inner(Instruction::Const { dst, src: n }));
                dst
            }
            Expr::BinOp { op, lhs, rhs } => {
                let lhs = self.lower_expr(*lhs);
                let rhs = self.lower_expr(*rhs);
                let dst = self.mk_var("_t");
                self.tv.push(Inner(Instruction::Arith { op, dst, lhs, rhs }));
                dst
            }
            Expr::Negate(e) => {
                // not the most efficient method, but it works
                self.lower_expr(Expr::BinOp { op: BOp::Sub, lhs: Box::new(Expr::Const(0)), rhs: e })
            }
        }
    }

    fn mk_var(&mut self, prefix: &str) -> Id {
        self.fresh_ctr += 1;
        let x = id(&format!("{prefix}_{}", self.fresh_ctr));
        self.decl.insert(x);
        x
    }

    fn mk_label(&mut self) -> Id {
        self.bb_ctr += 1;
        id(&format!("lbl{}", self.bb_ctr))
    }
}

fn construct_cfg(tv: Vec<TvEntry>) -> Map<Id, Block> {
    let mut tv_iter = tv.iter();

    let mut grammar: Map<Id, Block> = Map::new();

    let mut curr_block = match tv_iter.next() {
        Some(Label(id)) => id,
        _ => { return grammar; }
    };

    let mut insn: Vec<Instruction> = vec![];

    for channel in tv_iter {
        match channel {
            Label(id) => {
                curr_block = id;
            },
            Inner(ins) => {
                insn.push(ins.clone());
            },
            Term(term) => {
                grammar.insert(*curr_block, Block { insn, term: term.clone() });
                insn = vec![];
            }
        }
    }

    grammar
}

// fn main() {

// }
