use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::map;
use nom::IResult;
use nom::lib::std::collections::{VecDeque};
use nom::multi::{count, separated_list0};
use nom::sequence::{preceded, separated_pair, terminated};
use std::collections::HashSet;

fn main() {
    let mut input = String::default();
    let mut file = File::open("input/day22.txt").unwrap();
    file.read_to_string(&mut input).unwrap();

    let (_, (deck1, deck2)) = parse_input(&input).unwrap();

    let deck1 = VecDeque::from(deck1);
    let deck2 = VecDeque::from(deck2);

    let (_, winning_deck) = combat(deck1.clone(), deck2.clone());
    println!("Part 1: {}", compute_score(winning_deck));

    let (_, winning_deck) = recursive_combat(deck1.clone(), deck2.clone());
    println!("Part 2: {}", compute_score(winning_deck));
}

fn compute_score(deck: VecDeque<usize>) -> usize {
    deck.iter().rev().enumerate()
        .map(|(index, card)| (index + 1) * card)
        .sum()
}

fn combat(mut deck1: VecDeque<usize>, mut deck2: VecDeque<usize>) -> (bool, VecDeque<usize>) {
    while !deck1.is_empty() && !deck2.is_empty() {
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        let player1_wins = card1 > card2;

        if player1_wins {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    if deck1.is_empty() {
        (false, deck2)
    } else {
        (true, deck1)
    }
}

fn recursive_combat(
    mut deck1: VecDeque<usize>,
    mut deck2: VecDeque<usize>,
) -> (bool, VecDeque<usize>) {
    let mut prev_rounds = HashSet::new();

    while !deck1.is_empty() && !deck2.is_empty() {
        // println!("Player 1's deck: {}", deck1.iter().join(", "));
        // println!("Player 2's deck: {}", deck2.iter().join(", "));

        if !prev_rounds.insert((deck1.clone(), deck2.clone())) {
            return (true, deck1);
        }

        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        let player1_wins = if card1 <= deck1.len() && card2 <= deck2.len() {
            recursive_combat(
                deck1.iter().take(card1).cloned().collect(),
                deck2.iter().take(card2).cloned().collect(),
            ).0
        } else {
            card1 > card2
        };

        if player1_wins {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    if deck1.is_empty() {
        (false, deck2)
    } else {
        (true, deck1)
    }
}

fn parse_input(input: &str) -> IResult<&str, (Vec<usize>, Vec<usize>)> {
    separated_pair(
        preceded(
            terminated(tag("Player 1:"), line_ending),
            separated_list0(
                line_ending,
                map(digit1, |s| usize::from_str(s).unwrap()),
            ),
        ),
        count(line_ending, 2),
        preceded(
            terminated(tag("Player 2:"), line_ending),
            separated_list0(
                line_ending,
                map(digit1, |s| usize::from_str(s).unwrap()),
            ),
        ),
    )(input)
}