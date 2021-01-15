use crate::utils::{self, *};

lazy_static! {
    static ref INPUT: Vec<String> = utils::read_input_lines("day07");
}

type Bag = usize;

#[derive(Default, Clone)]
struct BagRule {
    count: i32,
    bag: Bag,
}

#[derive(Default, Clone)]
struct LuggageRule {
    bag: Bag,
    inner_bag_rules: Vec<BagRule>,
}

type EnumMap<'a> = HashMap<(&'a str, &'a str), usize>;

fn map_enum<'a>(m: &mut EnumMap<'a>, key: (&'a str, &'a str)) -> usize {
    let n = m.len();
    *m.entry(key).or_insert(n)
}

fn parse_luggage_rule<'a>(bag_map: &mut EnumMap<'a>, s: &'a str) -> LuggageRule {
    let mut parts = s.split(' ');
    let bag = map_enum(bag_map, (parts.next().unwrap(), parts.next().unwrap()));
    parts.next();
    parts.next();

    let mut inner_bag_rules = Vec::new();
    while let Some(n) = parts.next() {
        if n == "no" {
            break;
        }
        let count = n.parse().unwrap();
        let bag = map_enum(bag_map, (parts.next().unwrap(), parts.next().unwrap()));
        inner_bag_rules.push(BagRule { count, bag });

        parts.next();
    }
    LuggageRule {
        bag,
        inner_bag_rules,
    }
}

fn read_rules() -> (Vec<LuggageRule>, EnumMap<'static>) {
    let mut bag_map = EnumMap::with_capacity(INPUT.len());
    let mut all_bags = vec![LuggageRule::default(); INPUT.len()];
    for s in INPUT.iter() {
        let lr = parse_luggage_rule(&mut bag_map, s.as_str());
        let bag = lr.bag;
        all_bags[bag] = lr;
    }
    (all_bags, bag_map)
}

fn has_bag(
    memo: &mut HashMap<Bag, bool>,
    rules: &[LuggageRule],
    bag: Bag,
    search_bag: Bag,
) -> bool {
    if let Some(&v) = memo.get(&bag) {
        return v;
    }
    if bag == search_bag {
        return true;
    }
    let v = rules[bag]
        .inner_bag_rules
        .iter()
        .any(|b| has_bag(memo, rules, b.bag, search_bag));
    memo.insert(bag, v);
    v
}

pub fn part1() -> usize {
    let (rules, bag_map) = read_rules();
    let shiny_gold_bag = *bag_map.get(&("shiny", "gold")).unwrap();
    let mut c = 0;
    let mut memo = HashMap::new();
    for lr in rules.iter() {
        if has_bag(&mut memo, &rules, lr.bag, shiny_gold_bag) {
            c += 1;
        }
    }
    // Gold bag counts itself.
    c - 1
}

fn count_bags(memo: &mut HashMap<Bag, usize>, rules: &[LuggageRule], bag: Bag) -> usize {
    if let Some(v) = memo.get(&bag) {
        return *v;
    }
    let mut tot = 0;
    for inner_bag_rule in rules[bag].inner_bag_rules.iter() {
        tot += (inner_bag_rule.count as usize) * (1 + count_bags(memo, rules, inner_bag_rule.bag))
    }
    memo.insert(bag, tot);
    tot
}

pub fn part2() -> usize {
    let (rules, bag_map) = read_rules();
    let shiny_gold_bag = *bag_map.get(&("shiny", "gold")).unwrap();
    count_bags(&mut HashMap::new(), &rules, shiny_gold_bag)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn run_part1() {
        assert_eq!(part1(), 246);
    }

    #[test]
    fn run_part2() {
        assert_eq!(part2(), 2976);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part2);
    }
}
