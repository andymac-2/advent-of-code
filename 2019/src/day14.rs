use crate::parsers::*;
use std::collections::{HashMap, HashSet};

fn ws(s: &[u8]) -> ParseResult<()> {
    void(take_while_p(|c| c == b' ' || c == b'\t'))(s)
}

fn newline(s: &[u8]) -> ParseResult<()> {
    void(satisfy(|c| c == b'\n'))(s)
}

fn number(s: &[u8]) -> ParseResult<u64> {
    let (s, num_bytes) = take_while1_p(|c| c.is_ascii_digit())(s)?;
    let (s, _) = ws(s)?;

    let num_str = std::str::from_utf8(num_bytes).unwrap();
    let number = num_str.parse().unwrap();
    Some((s, number))
}

fn identifier(s: &[u8]) -> ParseResult<&[u8]> {
    let (s, result) = take_while1_p(|c| c.is_ascii_alphabetic())(s)?;
    let (s, _) = ws(s)?;
    Some((s, result))
}

fn arrow(s: &[u8]) -> ParseResult<()> {
    let (s, _) = void(chunk(b"=>"))(s)?;
    let (s, _) = ws(s)?;
    Some((s, ()))
}

fn comma(s: &[u8]) -> ParseResult<()> {
    let (s, _) = void(satisfy(|c| c == b','))(s)?;
    let (s, _) = ws(s)?;
    Some((s, ()))
}

fn quantity(s: &[u8]) -> ParseResult<Quantity> {
    let (s, num) = number(s)?;
    let (s, id) = identifier(s)?;
    let id_str = std::str::from_utf8(id).unwrap();
    Some((s, Quantity::new(num, id_str)))
}

fn rule(s: &[u8]) -> ParseResult<Rule> {
    let (s, quantities) = sep_by(quantity, comma)(s)?;
    let (s, _) = arrow(s)?;
    let (s, result) = quantity(s)?;
    Some((s, Rule::new(result, quantities)))
}

fn rules(s: &[u8]) -> ParseResult<Vec<Rule>> {
    sep_by(rule, newline)(s)
}

#[derive(Clone, Copy, Debug)]
struct Quantity<'a> {
    name: &'a str,
    number: u64,
}
impl<'a> Quantity<'a> {
    fn new(number: u64, name: &'a str) -> Self {
        Quantity { name, number }
    }
}
impl<'a> std::ops::Mul<u64> for Quantity<'a> {
    type Output = Quantity<'a>;
    fn mul(self, rhs: u64) -> Self {
        Quantity::new(self.number * rhs, self.name)
    }
}

struct Rule<'a> {
    result: Quantity<'a>,
    inputs: Vec<Quantity<'a>>,
}
impl<'a> Rule<'a> {
    fn new(result: Quantity<'a>, inputs: Vec<Quantity<'a>>) -> Self {
        Rule { result, inputs }
    }
}

const FUEL: &str = "FUEL";
const ORE: &str = "ORE";
struct Reagent<'a> {
    inputs: Vec<Quantity<'a>>,
    quantity: Option<u64>,
    outputs: Vec<&'a str>,
}
impl<'a> Reagent<'a> {
    fn get_ingredients(&self, amount: u64) -> Vec<Quantity<'a>> {
        let batch_quantity = self.quantity.unwrap();
        let multiple = (amount + batch_quantity - 1) / batch_quantity;

        self.inputs
            .iter()
            .map(|reagent| *reagent * multiple)
            .collect()
    }
    fn is_ready(&self, processed_reagents: &HashSet<&'a str>) -> bool {
        self.outputs
            .iter()
            .all(|output| processed_reagents.contains(output))
    }
}
impl<'a> Default for Reagent<'a> {
    fn default() -> Self {
        Reagent {
            inputs: Vec::new(),
            quantity: None,
            outputs: Vec::new(),
        }
    }
}

pub struct ReagentGraph<'a> {
    map: HashMap<&'a str, Reagent<'a>>,
}
impl<'a> ReagentGraph<'a> {
    fn from_rules(rules: Vec<Rule<'a>>) -> Self {
        let mut this = ReagentGraph {
            map: HashMap::new(),
        };

        for rule in rules.into_iter() {
            this.add_rule(rule)
        }

        this
    }
    fn add_rule(&mut self, rule: Rule<'a>) {
        for input in rule.inputs.iter() {
            let reagent = self.get_mut_reagent(input.name);
            reagent.outputs.push(rule.result.name);
        }

        let current = self.get_mut_reagent(rule.result.name);
        current.quantity = Some(rule.result.number);
        current.inputs = rule.inputs;
    }

    fn get_mut_reagent(&mut self, name: &'a str) -> &mut Reagent<'a> {
        self.map.entry(name).or_default()
    }

    fn get_ore(&self, num_ore: u64) -> u64 {
        let mut processed_reagents = HashSet::new();
        let mut ready_reagents = vec![FUEL];
        let mut current_reagents = HashMap::new();
        current_reagents.insert(FUEL, num_ore);
        // start with one fuel in the reagents list

        // while the reagents list does not only contain ore
        while let Some(ready_reagent) = ready_reagents.pop() {
            let node = &self.map[ready_reagent];
            if !node.is_ready(&processed_reagents) {
                continue;
            }

            let quantity = match current_reagents.remove(ready_reagent) {
                Some(quantity) => quantity,
                None => continue,
            };

            if ready_reagent == ORE {
                return quantity;
            }

            processed_reagents.insert(ready_reagent);

            let ingredients = node.get_ingredients(quantity);
            for ingredient in ingredients.into_iter() {
                ready_reagents.push(ingredient.name);

                let ingredient_quantity = current_reagents.entry(ingredient.name).or_insert(0);
                *ingredient_quantity += ingredient.number;
            }
        }
        unreachable!()
    }
}

pub fn part1(input: &[u8]) -> u64 {
    let rules = rules(input).unwrap().1;
    let graph = ReagentGraph::from_rules(rules);

    graph.get_ore(1)
}

const TRILLION: u64 = 1_000_000_000_000;
pub fn part2(input: &[u8]) -> u64 {
    let rules = rules(input).unwrap().1;
    let graph = ReagentGraph::from_rules(rules);

    let mut lower = 0;
    let mut upper = 10_000_000_000;
    while lower < upper {
        let middle = (lower + upper + 1) / 2;
        let ore = graph.get_ore(middle);

        if ore > TRILLION {
            upper = middle - 1;
        } else {
            lower = middle;
        }
    }
    lower
}

pub fn start() {
    let buffer = std::fs::read_to_string("./inputs/day14.txt")
        .expect("Something went wrong reading the file");

    println!("Program Output: {:?}", part2(buffer.as_bytes()));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT1: &[u8] = b"\
    9 ORE => 2 A\n\
    8 ORE => 3 B\n\
    7 ORE => 5 C\n\
    3 A, 4 B => 1 AB\n\
    5 B, 7 C => 1 BC\n\
    4 C, 1 A => 1 CA\n\
    2 AB, 3 BC, 4 CA => 1 FUEL\n";

    #[test]
    fn example_1() {
        assert_eq!(part1(INPUT1), 165);
    }
}
