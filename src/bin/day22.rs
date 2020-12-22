use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::map;
use nom::IResult;
use nom::multi::{count, separated_list0};
use nom::sequence::{preceded, separated_pair, terminated};
use tinyvec::{Array, ArrayVec};

trait ArrayVecDeque<A: Array> {
    fn pop_front(&mut self) -> Option<A::Item>;
    fn pop_back(&mut self) -> Option<A::Item>;

    fn push_front(&mut self, el: A::Item);
    fn push_back(&mut self, el: A::Item);
}

impl<A: Array> ArrayVecDeque<A> for ArrayVec<A> {
    fn pop_front(&mut self) -> Option<A::Item> {
        if self.len() > 0 {
            Some(self.remove(0))
        } else {
            None
        }
    }

    fn pop_back(&mut self) -> Option<A::Item> {
        self.pop()
    }

    fn push_front(&mut self, el: A::Item) {
        self.insert(0, el)
    }

    fn push_back(&mut self, el: A::Item) {
        self.push(el)
    }
}

type Deck = ArrayVec<[usize; 64]>;

fn main() {
    let mut input = String::default();
    let mut file = File::open("input/day22.txt").unwrap();
    file.read_to_string(&mut input).unwrap();

    let (_, (deck1, deck2)) = parse_input(&input).unwrap();

    let deck1 = Deck::from_iter(deck1.into_iter());
    let deck2 = Deck::from_iter(deck2.into_iter());

    let (_, winning_deck) = combat(deck1, deck2);
    println!("Part 1: {}", compute_score(winning_deck));

    let (_, winning_deck) = recursive_combat(deck1, deck2);
    println!("Part 2: {}", compute_score(winning_deck));
}

fn compute_score(deck: Deck) -> usize {
    deck.iter().rev().enumerate()
        .map(|(index, card)| (index + 1) * card)
        .sum()
}

fn combat(mut deck1: Deck, mut deck2: Deck) -> (bool, Deck) {
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
    mut deck1: Deck,
    mut deck2: Deck,
) -> (bool, Deck) {
    let mut prev_rounds = HashSet::new();

    while !deck1.is_empty() && !deck2.is_empty() {
        if !prev_rounds.insert((deck1.clone(), deck2.clone())) {
            return (true, deck1);
        }

        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        let player1_wins = if card1 <= deck1.len() && card2 <= deck2.len() {
            let mut sub_deck1 = deck1.clone();
            let mut sub_deck2 = deck2.clone();

            sub_deck1.truncate(card1);
            sub_deck2.truncate(card2);

            recursive_combat(
                sub_deck1,
                sub_deck2
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