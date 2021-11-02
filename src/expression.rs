use pest::error::Error as PestError;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::PrecClimber;
use pest::Parser;
use std::fmt;
use std::i64;
use std::num::ParseIntError;
use std::ops::Index;
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "expression.peg"]
struct CommandParser;

/// An Expr is either a node (which corresponds to a binary operation) or a leaf (which corresponds
/// to a number).
#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    BinOp(BinOpExpr),
    Num(i64),
    Ans,
}

/// An Op is a binary operator.
#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    LShift,
    RShift,
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
            "%" => Ok(Op::Rem),
            "<<" => Ok(Op::LShift),
            ">>" => Ok(Op::RShift),
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

impl Index<usize> for SetDirective {
    type Output = String;

    fn index(&self, idx: usize) -> &String {
        &self.args[idx]
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
pub fn parse_line<T: AsRef<str>>(line: T) -> Result<Command, PestError<Rule>> {
    let comm = CommandParser::parse(Rule::line, line.as_ref())?.next();
    if comm.is_none() {
        return Ok(Command::Empty);
    }
    Ok(parse_comm(comm.unwrap().into_inner().next().unwrap()))
}

fn parse_comm(pair: Pair<Rule>) -> Command {
    match pair.as_rule() {
        Rule::expr => Command::Expr(parse_expr(pair.into_inner())),
        Rule::set_directive => Command::Set(SetDirective {
            args: pair.as_str().split(' ').skip(1).map(String::from).collect(),
        }),
        _ => unreachable!(),
    }
}

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use pest::prec_climber::Assoc::*;
        use pest::prec_climber::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(lshift, Left) | Operator::new(rshift, Left),
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left) | Operator::new(rem, Left),
        ])
    };
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Radix {
    Bin,
    Oct,
    Dec,
    Hex,
}

impl Radix {
    fn numeric_radix(&self) -> u32 {
        match self {
            Radix::Bin => 2,
            Radix::Oct => 8,
            Radix::Dec => 10,
            Radix::Hex => 16,
        }
    }
}

fn parse_num(mut s: &str, radix: Radix) -> Result<i64, ParseIntError> {
    if radix != Radix::Dec {
        s = &s[2..];
    }
    let num_str = s.replace('_', "");
    i64::from_str_radix(&num_str, radix.numeric_radix())
}

fn parse_expr(expression: Pairs<Rule>) -> Expr {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::number => parse_expr(pair.into_inner()),
            Rule::dec => Expr::Num(parse_num(pair.as_str(), Radix::Dec).unwrap()),
            Rule::hex => Expr::Num(parse_num(pair.as_str(), Radix::Hex).unwrap()),
            Rule::oct => Expr::Num(parse_num(pair.as_str(), Radix::Oct).unwrap()),
            Rule::bin => Expr::Num(parse_num(pair.as_str(), Radix::Bin).unwrap()),
            Rule::ans => Expr::Ans,
            Rule::expr => parse_expr(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: Expr, op: Pair<Rule>, rhs: Expr| {
            let op = match op.as_rule() {
                Rule::add => Op::Add,
                Rule::subtract => Op::Sub,
                Rule::multiply => Op::Mul,
                Rule::divide => Op::Div,
                Rule::rem => Op::Rem,
                Rule::lshift => Op::LShift,
                Rule::rshift => Op::RShift,
                _ => unreachable!(),
            };
            Expr::BinOp(BinOpExpr {
                left: Box::new(lhs),
                right: Box::new(rhs),
                op,
            })
        },
    )
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

    impl std::error::Error for EvalError {}

    pub fn eval_expr(expr: &Expr, ans: i64) -> Result<i64, EvalError> {
        match &expr {
            Expr::Num(num) => Ok(*num),
            Expr::BinOp(expr) => {
                let left = eval_expr(expr.left.as_ref(), ans)?;
                let right = eval_expr(expr.right.as_ref(), ans)?;
                match expr.op {
                    Op::Add => Ok(left + right),
                    Op::Sub => Ok(left - right),
                    Op::Mul => Ok(left * right),
                    Op::LShift => Ok(left << right),
                    Op::RShift => Ok(left >> right),
                    Op::Div => {
                        if right == 0 {
                            Err(EvalError(String::from("Cannot divide by 0")))
                        } else {
                            Ok(left / right)
                        }
                    }
                    Op::Rem => {
                        if right == 0 {
                            Err(EvalError(String::from("Cannot divide by 0")))
                        } else {
                            Ok(left % right)
                        }
                    }
                }
            }
            Expr::Ans => Ok(ans),
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
            Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), 22),
            _ => panic!("Should have parsed to an expr"),
        };
        let expr2_str = "2 * (5 + 6)";
        match parse_line(expr2_str).unwrap() {
            Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), 22),
            _ => panic!("Should have parsed to an expr"),
        };
        let expr3_str = "3 * (9 + 6) - 4";
        match parse_line(expr3_str).unwrap() {
            Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), 41),
            _ => panic!("Should have parsed to an expr"),
        };
        let expr4_str = "6-57*(18+4/73)+38 *  124";
        match parse_line(expr4_str).unwrap() {
            Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), 3692),
            _ => panic!("Should have parsed to an expr"),
        };
        let expr5_str = "2 + (((7 * 2) - 4) / 2) + 8 * 9 / 4";
        match parse_line(expr5_str).unwrap() {
            Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), 25),
            _ => panic!("Should have parsed to an expr"),
        };
        let expr6_str = "(3 + 2) - 1 / 1 * 3 + 5 * 4 / 10 - 1";
        match parse_line(expr6_str).unwrap() {
            Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), 3),
            _ => panic!("Should have parsed to an expr"),
        };
        let expr7_str = "8 / 2 * 3 - 9 - 6 * (15 / 3 / 5)";
        match parse_line(expr7_str).unwrap() {
            Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), -3),
            _ => panic!("Should have parsed to an expr"),
        };
        let expr8_str = "24 / (2 * (12 / 4)) - ((8 * 3) / 6)";
        match parse_line(expr8_str).unwrap() {
            Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), 0),
            _ => panic!("Should have parsed to an expr"),
        };
        let expr9_str = "3 * 512 >> 4 - 2";
        match parse_line(expr9_str).unwrap() {
            Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), 384),
            _ => panic!("Should have parsed to an expr"),
        };
        let expr10_str = "3 * (512 >> 4) - 2";
        match parse_line(expr10_str).unwrap() {
            Command::Expr(expr) => assert_eq!(eval_expr(&expr, 0).unwrap(), 94),
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
        let hex_str3 = "0xFACE_A0CE";
        assert_eq!(
            parse_line(hex_str3).unwrap(),
            Command::Expr(Expr::Num(4207845582))
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
        let oct_str3 = "0o1232_34_4";
        assert_eq!(
            parse_line(oct_str3).unwrap(),
            Command::Expr(Expr::Num(341220))
        );
    }

    #[test]
    fn bin_parse() {
        let bin_str1 = "0b1010";
        assert_eq!(parse_line(bin_str1).unwrap(), Command::Expr(Expr::Num(10)));
        let bin_str1 = "0b10100101";
        assert_eq!(parse_line(bin_str1).unwrap(), Command::Expr(Expr::Num(165)));
        let bin_str3 = "0b10_10_01____01";
        assert_eq!(parse_line(bin_str3).unwrap(), Command::Expr(Expr::Num(165)));
    }

    #[test]
    fn dec_parse() {
        let dec_str1 = "1234_5678";
        assert_eq!(
            parse_line(dec_str1).unwrap(),
            Command::Expr(Expr::Num(12345678))
        );
    }
}
