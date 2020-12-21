use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending};
use nom::{IResult, Parser};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, separated_pair};

fn main() {
    let mut input = String::default();
    let mut file = File::open("input/day21.txt").unwrap();
    file.read_to_string(&mut input).unwrap();

    let (_, foods) = parse_input(&input).unwrap();

    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut ingredients_count: HashMap<&str, usize> = HashMap::new();

    for (ingredients, allergens) in foods {
        for ingredient in &ingredients {
            *ingredients_count.entry(ingredient).or_default() += 1;
        }

        let mut ingredients_set: HashSet<&str> = ingredients.iter().cloned().collect();
        for allergen in allergens {
            let new_set: HashSet<&str> = map.entry(allergen)
                .or_insert(ingredients_set.clone())
                .intersection(&ingredients_set)
                .cloned()
                .collect();
            map.insert(allergen, new_set);
        }
    }

    let mut clear_ingredients: HashSet<&str> = ingredients_count.keys().cloned().collect();

    for (allergen, ingredients) in map {
        clear_ingredients = clear_ingredients.difference(&ingredients).cloned().collect();
        println!("{}: {}", allergen, ingredients.iter().join(", "))
    }
    println!();

    let count_part1: usize = clear_ingredients.iter().map(|ingredient| {
        ingredients_count[ingredient]
    }).sum();
    println!("Part 1: {}", count_part1);
    println!("Part 2: (solve manually)");
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Vec<&str>, Vec<&str>)>> {
    separated_list0(line_ending, parse_food)(input)
}

fn parse_food(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    separated_pair(
        separated_list1(tag(" "), alpha1),
        tag(" "),
        delimited(
            tag("(contains "),
            separated_list1(tag(", "), alpha1),
            tag(")"),
        ),
    )(input)
}