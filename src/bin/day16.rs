use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::RangeInclusive;
use std::str::FromStr;

use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{digit1, line_ending};
use nom::combinator::map;
use nom::IResult;
use nom::lib::std::collections::BTreeSet;
use nom::multi::{many1, separated_list0, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};

type Rules = HashMap<String, (RangeInclusive<u16>, RangeInclusive<u16>)>;
type Ticket = Vec<u16>;

fn main() {
    let mut file = File::open("input/day16.txt").unwrap();
    let mut input = String::default();
    file.read_to_string(&mut input).unwrap();

    let (_, (rules, your_ticket, nearby_tickets)) = parse_input(&input).unwrap();

    let mut ranges: Vec<RangeInclusive<u16>> = vec![];
    for (_, rule) in &rules {
        ranges.push(rule.0.clone());
        ranges.push(rule.1.clone());
    }

    let mut error_rate = 0;
    let mut valid_tickets = vec![];

    for ticket in &nearby_tickets {
        let mut is_valid = true;

        for field in ticket {
            let mut is_valid_field = false;

            for range in &ranges {
                if range.contains(field) {
                    is_valid_field = true;
                    break;
                }
            }

            if !is_valid_field {
                error_rate += field;
                is_valid = false;
            }
        }

        if is_valid {
            valid_tickets.push(ticket);
        }
    }

    println!("Part 1: {}", error_rate);

    let mut possible_fields_map: Vec<BTreeSet<String>> = vec![];
    for (_, _) in &rules {
        possible_fields_map.push(rules.keys().cloned().collect());
    }

    for ticket in valid_tickets {
        for (idx, value) in ticket.iter().enumerate() {
            let possible_fields = possible_fields_map[idx].clone();
            let mut invalid_fields = vec![];

            for field_name in possible_fields.iter() {
                let (r1, r2) = rules.get(field_name).unwrap().clone();
                if !r1.contains(value) && !r2.contains(value) {
                    invalid_fields.push(field_name);
                }
            }

            for field_name in invalid_fields {
                possible_fields_map[idx].remove(field_name);
            }
        }
    }

    let mut final_field_names = vec![String::default(); rules.len()];
    while let Some(solo_map) = possible_fields_map.iter().find_map(|set| if set.len() == 1 { Some(set.clone()) } else { None }) {
        for i in 0..possible_fields_map.len() {
            if possible_fields_map[i].len() == 1 {
                final_field_names[i] = possible_fields_map[i].iter().nth(0).unwrap().to_string();
            }

            possible_fields_map[i] = possible_fields_map[i].difference(&solo_map).cloned().collect()
        }
    }

    let mut answer_part2: u64 = 1;
    for (idx, name) in final_field_names.iter().enumerate() {
        if name.starts_with("departure") {
            answer_part2 *= your_ticket[idx] as u64;
        }
    }
    println!(Part 2: "{}", answer_part2);
}

fn parse_input(input: &str) -> IResult<&str, (Rules, Ticket, Vec<Ticket>)> {
    tuple((
        terminated(parse_rules, line_ending),
        delimited(
            terminated(tag("your ticket:"), line_ending),
            terminated(parse_ticket, line_ending),
            line_ending,
        ),
        preceded(
            terminated(tag("nearby tickets:"), line_ending),
            separated_list0(line_ending, parse_ticket),
        )
    ))(input)
}

fn parse_rules(input: &str) -> IResult<&str, Rules> {
    map(
        many1(terminated(parse_rule, line_ending)),
        |tuples| tuples.into_iter().collect(),
    )(input)
}

fn parse_rule(input: &str) -> IResult<&str, (String, (RangeInclusive<u16>, RangeInclusive<u16>))> {
    separated_pair(
        map(take_until(":"), |s: &str| s.to_string()),
        tag(": "),
        separated_pair(
            parse_range,
            tag(" or "),
            parse_range,
        ),
    )(input)
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u16>> {
    map(
        separated_pair(digit1, tag("-"), digit1),
        |(s1, s2)| {
            let n1 = u16::from_str(s1).unwrap();
            let n2 = u16::from_str(s2).unwrap();
            n1..=n2
        },
    )(input)
}

fn parse_ticket(input: &str) -> IResult<&str, Ticket> {
    separated_list1(
        tag(","),
        map(digit1, |s| u16::from_str(s).unwrap()),
    )(input)
}