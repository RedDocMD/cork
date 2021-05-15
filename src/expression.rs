use pest::error::Error;
use pest::Parser;

#[derive(Parser)]
#[grammar = "expression.pest"]
struct CommandrParser;

/// An Expr is either a node (which corresponds to a binary operation) or a leaf (which corresponds
/// to a number).
///
/// Eg: The expression (5 + 6) * 2 will correspond to:
/// ```
/// Expr::BinOp(
///     BinOpExpr{
///         Box::new(Expr::BinOp(
///                     BinOpExpr{
///                         Box::new(Expr::Num(5)),
///                         Box::new(Expr::Num(6)),
///                         Op::Add,
///                     }),
///         Box::new(Expr::Num(2)),
///         Op::Mul,
///     }
/// )
/// ```
pub enum Expr {
    BinOp(BinOpExpr),
    Num(i64),
}

/// An Op is a binary operator.
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

/// A BinOpExpr is an expr which has two operands and an operator.
/// The two operands might also be expressions.
pub struct BinOpExpr {
    left: Box<Expr>,
    right: Box<Expr>,
    op: Op,
}

/// A SetDirective is a command of the form "set [args]+".
pub struct SetDirective {
    args: Vec<String>,
}

/// A Command is a one line worth of input from the user.
/// It can either be a SetDirective or an Expr.
/// As an escape-hatch, there is also an empty command.
pub enum Command {
    Expr(Expr),
    Set(SetDirective),
    Empty,
}

pub fn parse_line<T: AsRef<str>>(line: T) -> Result<Command, Error<Rule>> {
    let comm = CommandrParser::parse(Rule::line, line.as_ref())?.next();
    if let None = comm {
        return Ok(Command::Empty);
    }
    let comm = comm.unwrap();
}
