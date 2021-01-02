use std::collections::HashMap;

use regex::Regex;

use crate::utils;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Bag {
    attr: String,
    color: String,
}

struct BagRule {
    count: i32,
    bag: Bag,
}

struct LuggageRule {
    bag: Bag,
    inner_bag_rules: Vec<BagRule>,
}

lazy_static! {
    static ref LUGGAGE_REGEX: Regex = Regex::new(r"^(?:(\d+) )?(\w+) (\w+) bags?\.?$").unwrap();
}

fn parse_luggage_rule<S: AsRef<str>>(s: S) -> LuggageRule {
    let parts = s.as_ref().split(" contain ").collect::<Vec<_>>();
    let captures = LUGGAGE_REGEX.captures(parts[0]).unwrap();
    let attr = captures[2].to_string();
    let color = captures[3].to_string();
    let mut inner_bag_rules = Vec::new();
    if parts[1] != "no other bags." {
        for inner_part in parts[1].split(", ") {
            let captures = LUGGAGE_REGEX.captures(inner_part).unwrap();
            inner_bag_rules.push(BagRule {
                count: captures[1].to_string().parse().unwrap(),
                bag: Bag {
                    attr: captures[2].to_string(),
                    color: captures[3].to_string(),
                },
            });
        }
    }
    LuggageRule {
        bag: Bag { attr, color },
        inner_bag_rules,
    }
}

fn read_rules() -> HashMap<Bag, LuggageRule> {
    utils::read_input_lines("day07")
        .into_iter()
        .map(parse_luggage_rule)
        .map(|lr| (lr.bag.clone(), lr))
        .collect()
}

fn has_bag(
    memo: &mut HashMap<Bag, bool>,
    rules: &HashMap<Bag, LuggageRule>,
    bag: &Bag,
    search_bag: &Bag,
) -> bool {
    if *bag == *search_bag {
        return true;
    }
    if let Some(v) = memo.get(bag) {
        return *v;
    }
    let v = rules
        .get(bag)
        .unwrap()
        .inner_bag_rules
        .iter()
        .any(|b| has_bag(memo, rules, &b.bag, search_bag));
    memo.insert(bag.clone(), v);
    v
}

pub fn part1() -> usize {
    let shiny_gold_bag = Bag {
        attr: "shiny".to_string(),
        color: "gold".to_string(),
    };
    let rules = read_rules();
    let mut c = 0;
    let mut memo = HashMap::new();
    for (b, _) in rules.iter() {
        if *b == shiny_gold_bag {
            continue;
        }
        if has_bag(&mut memo, &rules, b, &shiny_gold_bag) {
            c += 1;
        }
    }
    c
}

fn count_bags(
    memo: &mut HashMap<Bag, usize>,
    rules: &HashMap<Bag, LuggageRule>,
    bag: &Bag,
) -> usize {
    if let Some(v) = memo.get(bag) {
        return *v;
    }
    let mut tot = 0;
    for inner_bag_rule in rules.get(bag).unwrap().inner_bag_rules.iter() {
        tot += (inner_bag_rule.count as usize) * (1 + count_bags(memo, rules, &inner_bag_rule.bag))
    }
    memo.insert(bag.clone(), tot);
    tot
}

pub fn part2() -> usize {
    let rules = read_rules();
    let shiny_gold_bag = Bag {
        attr: "shiny".to_string(),
        color: "gold".to_string(),
    };
    count_bags(&mut HashMap::new(), &rules, &shiny_gold_bag)
}
