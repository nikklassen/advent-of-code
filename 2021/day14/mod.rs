use ahash::AHashMap;
use shared::utils;

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day14");
    static ref INPUT: Vec<String> = utils::read_input_lines("day14");
}

type Element = usize;

fn to_rule_key(c1: Element, c2: Element) -> usize {
    c1 * 26 + c2
}

type ExpandCacheKey = usize;

fn to_cache_key(c1: Element, c2: Element, step: usize) -> ExpandCacheKey {
    to_rule_key(c1, c2) * 100 + step
}

fn alpha_to_num(c: char) -> usize {
    c as usize - 'A' as usize
}

type ExpansionRules = Vec<usize>;

fn parse_input() -> (Vec<usize>, ExpansionRules) {
    let template_chars: Vec<char> = INPUT[0].chars().collect();
    let mut template = Vec::with_capacity(template_chars.len());
    for &c in template_chars.iter() {
        template.push(alpha_to_num(c));
    }
    let mut rule_map: Vec<usize> = vec![0; 26 * 26];
    for line in INPUT[2..].iter() {
        let mut parts = line.split(" -> ");
        let lhs = parts.next().unwrap().chars().collect::<Vec<char>>();
        let (c1, c2) = (lhs[0], lhs[1]);
        let rhs = parts.next().unwrap().chars().next().unwrap();
        rule_map[to_rule_key(alpha_to_num(c1), alpha_to_num(c2))] = alpha_to_num(rhs);
    }
    (template, rule_map)
}

type ElementCounts = [usize; 26];

fn merge_counts(mut left: ElementCounts, right: &ElementCounts) -> ElementCounts {
    for i in 0..left.len() {
        left[i] += right[i];
    }
    left
}

fn expand(
    cache: &mut AHashMap<ExpandCacheKey, ElementCounts>,
    rules: &ExpansionRules,
    c1: usize,
    c2: usize,
    step: usize,
) -> ElementCounts {
    let rule_key = to_rule_key(c1, c2);
    let cache_key = to_cache_key(c1, c2, step);
    if let Some(v) = cache.get(&cache_key) {
        return v.clone();
    }

    let to_insert = rules[rule_key];
    let mut counts = if step == 0 {
        let mut counts = [0; 26];
        counts[c1] += 1;
        counts[to_insert] += 1;
        counts[c2] += 1;
        counts
    } else {
        let left = expand(cache, rules, c1, to_insert, step - 1);
        let right = expand(cache, rules, to_insert, c2, step - 1);
        merge_counts(left, &right)
    };

    counts[to_insert] -= 1;
    cache.insert(cache_key, counts);
    counts
}

fn run(steps: usize) -> usize {
    let (template, rules) = parse_input();
    let mut cache = AHashMap::new();
    let mut acc: ElementCounts = [0; 26];
    for i in 1..template.len() {
        let mut counts = expand(&mut cache, &rules, template[i - 1], template[i], steps);
        if i > 1 {
            counts[template[i - 1]] -= 1;
        }
        acc = merge_counts(acc, &counts);
    }
    let mut min = usize::MAX;
    let mut max = 0;
    for &count in acc.iter() {
        if count == 0 {
            continue;
        }
        if count < min {
            min = count;
        }
        if count > max {
            max = count;
        }
    }
    max - min
}

pub fn part1() -> usize {
    run(10)
}

pub fn part2() -> usize {
    run(40)
}
