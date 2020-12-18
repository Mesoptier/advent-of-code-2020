use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::fold_many0;
use nom::sequence::{delimited, pair, preceded};

fn main() {
    let file = File::open("input/day18.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines().map(|l| l.unwrap()).collect_vec();

    println!("Part 1: {}", solve(&lines, false));
    println!("Part 2: {}", solve(&lines, true));
}

fn solve(lines: &Vec<String>, part2: bool) -> u64 {
    lines.iter().map(|input| {
        let (_, expr) = parse_expression(part2)(input).unwrap();
        expr.evaluate()
    }).sum()
}

#[derive(Debug, Clone)]
enum Expression {
    Number(u64),
    Sum(Box<Expression>, Box<Expression>),
    Product(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn evaluate(&self) -> u64 {
        match self {
            Expression::Number(n) => *n,
            Expression::Sum(left, right) => {
                left.evaluate() + right.evaluate()
            }
            Expression::Product(left, right) => {
                left.evaluate() * right.evaluate()
            }
        }
    }
}

fn parse_expression(part2: bool) -> impl Fn(&str) -> IResult<&str, Expression> {
    move |input| {
        if !part2 {
            let (input, left_expr) = parse_value(part2)(input)?;

            fold_many0(
                pair(parse_operator, parse_value(part2)),
                left_expr,
                |left_expr: Expression, (op, right_expr)| {
                    op(Box::new(left_expr), Box::new(right_expr))
                },
            )(input)
        } else {
            alt((
                parse_product(part2),
                parse_sum(part2),
                parse_value(part2),
            ))(input)
        }
    }
}

fn parse_product(part2: bool) -> impl Fn(&str) -> IResult<&str, Expression> {
    move |input| {
        let (input, left_expr) = parse_sum(part2)(input)?;

        fold_many0(
            preceded(tag(" * "), parse_sum(part2)),
            left_expr,
            |left, right| Expression::Product(Box::new(left), Box::new(right)),
        )(input)
    }
}

fn parse_sum(part2: bool) -> impl Fn(&str) -> IResult<&str, Expression> {
    move |input| {
        let (input, left_expr) = parse_value(part2)(input)?;

        fold_many0(
            preceded(tag(" + "), parse_value(part2)),
            left_expr,
            |left, right| Expression::Sum(Box::new(left), Box::new(right)),
        )(input)
    }
}

fn parse_value(part2: bool) -> impl Fn(&str) -> IResult<&str, Expression> {
    move |input| {
        alt((parse_number, parse_parens(part2)))(input)
    }
}

type Operator = fn(Box<Expression>, Box<Expression>) -> Expression;

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    alt((
        value(Expression::Sum as Operator, tag(" + ")),
        value(Expression::Product as Operator, tag(" * ")),
    ))(input)
}

fn parse_number(input: &str) -> IResult<&str, Expression> {
    map(
        digit1,
        |s| Expression::Number(u64::from_str(s).unwrap()),
    )(input)
}

fn parse_parens(part2: bool) -> impl Fn(&str) -> IResult<&str, Expression> {
    move |input| {
        delimited(tag("("), parse_expression(part2), tag(")"))(input)
    }
}