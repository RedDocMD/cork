use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use std::fmt;
use std::i64;
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "expression.pest"]
struct CommandParser;

/// An Expr is either a node (which corresponds to a binary operation) or a leaf (which corresponds
/// to a number).
#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    BinOp(BinOpExpr),
    Num(i64),
}

/// An Op is a binary operator.
#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub struct ParseOpError(String);

impl FromStr for Op {
    type Err = ParseOpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Sub),
            "*" => Ok(Op::Mul),
            "/" => Ok(Op::Div),
            _ => Err(ParseOpError(format!("{} is not an Op", s))),
        }
    }
}

/// A BinOpExpr is an expr which has two operands and an operator.
/// The two operands might also be expressions.
#[derive(Debug, Eq)]
pub struct BinOpExpr {
    left: Box<Expr>,
    right: Box<Expr>,
    op: Op,
}

impl PartialEq for BinOpExpr {
    fn eq(&self, rhs: &Self) -> bool {
        *self.left == *rhs.left && *self.right == *rhs.right && self.op == rhs.op
    }
}

/// A SetDirective is a command of the form "set [args]+".
#[derive(Debug, PartialEq, Eq)]
pub struct SetDirective {
    args: Vec<String>,
}

impl fmt::Display for SetDirective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "set {}", self.args.join(" "))
    }
}

/// A Command is a one line worth of input from the user.
/// It can either be a SetDirective or an Expr.
/// As an escape-hatch, there is also an empty command.
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Expr(Expr),
    Set(SetDirective),
    Empty,
}

/// parse_line takes in a user input, and parses it to a valid Command
/// or results in a parse error.
pub fn parse_line<T: AsRef<str>>(line: T) -> Result<Command, Error<Rule>> {
    let comm = CommandParser::parse(Rule::line, line.as_ref())?.next();
    if let None = comm {
        return Ok(Command::Empty);
    }
    Ok(parse_comm(comm.unwrap().into_inner().next().unwrap()))
}

fn parse_comm(pair: Pair<Rule>) -> Command {
    match pair.as_rule() {
        Rule::number => parse_comm(pair.into_inner().next().unwrap()),
        Rule::dec => Command::Expr(Expr::Num(pair.as_str().parse().unwrap())),
        Rule::hex => Command::Expr(Expr::Num(
            i64::from_str_radix(&pair.as_str()[2..], 16).unwrap(),
        )),
        Rule::oct => Command::Expr(Expr::Num(
            i64::from_str_radix(&pair.as_str()[2..], 8).unwrap(),
        )),
        Rule::bin => Command::Expr(Expr::Num(
            i64::from_str_radix(&pair.as_str()[2..], 2).unwrap(),
        )),
        Rule::expr => {
            let mut inner = pair.into_inner();
            if let Command::Expr(left_expr) = parse_comm(inner.next().unwrap()) {
                if let Some((op, right_expr)) = parse_expr_prime(inner.next().unwrap()) {
                    Command::Expr(Expr::BinOp(BinOpExpr {
                        left: Box::new(left_expr),
                        right: Box::new(right_expr),
                        op,
                    }))
                } else {
                    Command::Expr(left_expr)
                }
            } else {
                panic!("First part of expr must evaluate to an expr!");
            }
        }
        Rule::beta_expr => {
            let mut inner = pair.into_inner();
            let first = inner.next().unwrap();
            let first_str = first.as_str();
            if first_str == "(" {
                parse_comm(inner.next().unwrap())
            } else if first_str == "-" {
                match parse_comm(inner.next().unwrap()) {
                    Command::Expr(expr) => match expr {
                        Expr::Num(num) => Command::Expr(Expr::Num(-num)),
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            } else {
                parse_comm(first)
            }
        }
        Rule::set_directive => Command::Set(SetDirective {
            args: pair
                .into_inner()
                .skip(1)
                .map(|pair| String::from(pair.as_str()))
                .collect(),
        }),
        _ => unreachable!(),
    }
}

fn parse_expr_prime(pair: Pair<Rule>) -> Option<(Op, Expr)> {
    match pair.as_rule() {
        Rule::expr_prime => {
            if pair.as_str() == "" {
                None
            } else {
                let mut inner = pair.into_inner();
                let op: Op = inner.next().unwrap().as_str().parse().unwrap();
                if let Command::Expr(left_expr) = parse_comm(inner.next().unwrap()) {
                    if let Some((new_op, right_expr)) = parse_expr_prime(inner.next().unwrap()) {
                        let expr = Expr::BinOp(BinOpExpr {
                            left: Box::new(left_expr),
                            right: Box::new(right_expr),
                            op: new_op,
                        });
                        Some((op, expr))
                    } else {
                        Some((op, left_expr))
                    }
                } else {
                    panic!("Second part of expr_prime must be an expr");
                }
            }
        }
        _ => unreachable!(),
    }
}

pub mod eval {
    use std::fmt;

    use super::*;

    #[derive(Debug)]
    pub struct EvalError(String);

    impl fmt::Display for EvalError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    pub fn eval_expr(expr: &Expr) -> Result<i64, EvalError> {
        match &expr {
            Expr::Num(num) => Ok(*num),
            Expr::BinOp(expr) => {
                let left = eval_expr(expr.left.as_ref())?;
                let right = eval_expr(expr.right.as_ref())?;
                match expr.op {
                    Op::Add => Ok(left + right),
                    Op::Sub => Ok(left - right),
                    Op::Mul => Ok(left * right),
                    Op::Div => {
                        if right == 0 {
                            Err(EvalError(String::from("Cannot divide by 0")))
                        } else {
                            Ok(left / right)
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::eval::*;
    use super::*;

    #[test]
    fn test_expr_parse() {
        let expr1 = Expr::BinOp(BinOpExpr {
            left: Box::new(Expr::BinOp(BinOpExpr {
                left: Box::new(Expr::Num(5)),
                right: Box::new(Expr::Num(6)),
                op: Op::Add,
            })),
            right: Box::new(Expr::Num(2)),
            op: Op::Mul,
        });
        let expr_str1 = "(5 + 6) * 2";
        assert_eq!(parse_line(expr_str1).unwrap(), Command::Expr(expr1));
        let expr2 = Expr::BinOp(BinOpExpr {
            right: Box::new(Expr::BinOp(BinOpExpr {
                left: Box::new(Expr::Num(5)),
                right: Box::new(Expr::Num(6)),
                op: Op::Add,
            })),
            left: Box::new(Expr::Num(2)),
            op: Op::Mul,
        });
        let expr_str2 = "2 * (5 + 6)";
        assert_eq!(parse_line(expr_str2).unwrap(), Command::Expr(expr2));
    }

    #[test]
    fn test_expr_eval() {
        let expr1_str = "(5 + 6) * 2";
        match parse_line(expr1_str).unwrap() {
            Command::Expr(expr) => assert_eq!(eval_expr(&expr).unwrap(), 22),
            _ => panic!("Should have parsed to an expr"),
        };
    }

    #[test]
    fn hex_parse() {
        let hex_str1 = "0x1a";
        assert_eq!(parse_line(hex_str1).unwrap(), Command::Expr(Expr::Num(26)));
        let hex_str2 = "0xCAFE";
        assert_eq!(
            parse_line(hex_str2).unwrap(),
            Command::Expr(Expr::Num(51966))
        );
    }

    #[test]
    fn oct_parse() {
        let oct_str1 = "0o345";
        assert_eq!(parse_line(oct_str1).unwrap(), Command::Expr(Expr::Num(229)));
        let oct_str2 = "0o1232344";
        assert_eq!(
            parse_line(oct_str2).unwrap(),
            Command::Expr(Expr::Num(341220))
        );
    }

    #[test]
    fn bin_parse() {
        let bin_str1 = "0b1010";
        assert_eq!(parse_line(bin_str1).unwrap(), Command::Expr(Expr::Num(10)));
        let bin_str1 = "0b10100101";
        assert_eq!(parse_line(bin_str1).unwrap(), Command::Expr(Expr::Num(165)));
    }
}
