//! The tiny IR.

use std::fmt::Display;

use crate::common::*;
use crate::front::ast::BOp;

#[derive(Debug)]
pub struct Program {
    pub decl: Set<Id>,
    pub block: Map<Id, Block>,
}

#[derive(Debug)]
pub struct Block {
    pub insn: Vec<Instruction>,
    pub term: Terminator,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Copy { dst: Id, src: Id },
    Const { dst: Id, src: i64 },
    Arith { op: BOp, dst: Id, lhs: Id, rhs: Id },
    Read(Id),
    Print(Id),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Instruction::*;
        match self {
            Copy { dst, src } => write!(f, "{dst} = $copy {src}"),
            Const { dst, src } => write!(f, "{dst} = $const {src}"),
            Arith { op, dst, lhs, rhs } => write!(f, "{dst} = $arith {op} {lhs} {rhs}"),
            Read(x) => write!(f, "$read {x}"),
            Print(x) => write!(f, "$print {x}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Terminator {
    Exit,
    Jump(Id),
    Branch { guard: Id, tt: Id, ff: Id },
}

impl Display for Terminator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Terminator::*;
        match self {
            Exit => write!(f, "$exit"),
            Jump(lbl) => write!(f, "$jump {lbl}"),
            Branch { guard, tt, ff } => write!(f, "$branch {guard} {tt} {ff}"),
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "let ")?;
        for x in &self.decl {
            write!(f, "{x}, ")?;
        }
        writeln!(f)?;

        for (lbl, block) in &self.block {
            writeln!(f, "{lbl}:")?;
            for insn in &block.insn {
                writeln!(f, "    {insn}")?;
            }
            writeln!(f, "    {}", block.term)?;
        }

        Ok(())
    }
}
