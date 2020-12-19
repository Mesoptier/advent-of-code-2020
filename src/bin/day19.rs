use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, anychar, digit1, line_ending};
use nom::combinator::{all_consuming, map, value};
use nom::multi::{many1, many1_count, many_m_n, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated};

fn main() {
    let mut input = String::default();
    let mut file = File::open("input/day19.txt").unwrap();
    file.read_to_string(&mut input).unwrap();

    let (_, (rules, messages)) = parse_input(&input).unwrap();

    println!("Part 1: {}", count_matching(&messages, &rules));

    // Use custom 0th rule to mimic alternative rule 8 + 11 behavior
    let mut rules = rules.clone();
    rules.insert(0, Rule::Custom0);

    println!("Part 2: {}", count_matching(&messages, &rules));
}

fn count_matching(messages: &Vec<&str>, rules: &HashMap<usize, Rule>) -> usize {
    messages.iter().filter(|msg| {
        all_consuming(rules[&0].parser(&rules))(msg).is_ok()
    }).count()
}

#[derive(Clone, Debug)]
enum Rule {
    Char(char),
    Seq(Vec<usize>),
    Alt(Vec<usize>, Vec<usize>),
    Custom0,
}

impl Rule {
    fn parser<'a>(&'a self, rules: &'a HashMap<usize, Rule>) -> impl Fn(&str) -> IResult<&str, ()> + 'a {
        move |input| {
            match self {
                Rule::Char(c) => {
                    value((), tag(c.to_string().as_str()))(input)
                }
                Rule::Seq(seq) => {
                    seq.iter().fold(
                        Ok((input, ())),
                        |acc, idx| {
                            if let Ok((input, _)) = acc {
                                let rule = rules.get(idx).unwrap();
                                rule.parser(rules)(input)
                            } else {
                                acc
                            }
                        },
                    )
                }
                Rule::Alt(s1, s2) => {
                    alt((
                        Rule::Seq(s1.clone()).parser(rules),
                        Rule::Seq(s2.clone()).parser(rules),
                    ))(input)
                }
                Rule::Custom0 => {
                    let rule42 = rules.get(&42).unwrap();
                    let rule31 = rules.get(&31).unwrap();

                    let (input, _) = rule42.parser(rules)(input)?;
                    let (input, n) = many1_count(rule42.parser(rules))(input)?;
                    value((), many_m_n(1, n, rule31.parser(rules)))(input)
                }
            }
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, (HashMap<usize, Rule>, Vec<&str>)> {
    separated_pair(
        map(
            many1(terminated(parse_rule, line_ending)),
            |pairs| {
                pairs.iter().cloned().collect()
            },
        ),
        line_ending,
        separated_list1(line_ending, alpha1),
    )(input)
}

fn parse_rule(input: &str) -> IResult<&str, (usize, Rule)> {
    separated_pair(
        map(digit1, |s| usize::from_str(s).unwrap()),
        tag(": "),
        alt((
            // Rule::Char
            delimited(
                tag("\""),
                map(anychar, |c| Rule::Char(c)),
                tag("\""),
            ),
            // Rule::Alt
            map(
                separated_pair(parse_seq, tag(" | "), parse_seq),
                |(seq1, seq2)| Rule::Alt(seq1, seq2),
            ),
            // Rule::Seq
            map(parse_seq, |seq| Rule::Seq(seq)),
        )),
    )(input)
}

fn parse_seq(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(
        tag(" "),
        map(digit1, |s| usize::from_str(s).unwrap()),
    )(input)
}