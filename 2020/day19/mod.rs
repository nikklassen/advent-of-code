use crate::utils::{self, *};
use std::fmt;

lazy_static! {
    static ref INPUT: Vec<String> = utils::read_input_lines("day19");
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone)]
enum Rule {
    Or(Vec<Rule>),
    RuleNum(usize),
    Terminal(u8),
    ManyTill(usize, usize),
    And(Vec<Rule>),
}

impl Rule {
    pub fn unwrap<'a>(&'a self, rule_map: &'a [Rule]) -> &'a Rule {
        if let Rule::RuleNum(n) = self {
            &rule_map[*n]
        } else {
            self
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Rule::Or(rules) => {
                let mut vals = vec![];
                for rule in rules.iter() {
                    vals.push(rule.to_string());
                }
                "(".to_string() + &vals.join(" | ") + ")"
            }
            Rule::RuleNum(n) => n.to_string(),
            Rule::Terminal(c) => (*c as char).to_string(),
            Rule::ManyTill(n1, n2) => format!("{}+{}", n1, n2),
            Rule::And(rules) => {
                let mut vals = vec![];
                for rule in rules.iter() {
                    vals.push(rule.to_string());
                }
                vals.join(" ")
            }
        };
        write!(f, "{}", s)
    }
}

fn parse_case(s: &str) -> Vec<usize> {
    s.split(' ').map(|part| part.parse().unwrap()).collect()
}

fn case_to_rule(case: Vec<usize>) -> Rule {
    Rule::And(case.into_iter().map(Rule::RuleNum).collect())
}

fn parse_rule(s: &str) -> (usize, Rule) {
    let mut parts = s.split(": ");
    let rule_num = parts.next().unwrap().parse().unwrap();
    let raw_rule = parts.next().unwrap();
    let rule = if raw_rule.starts_with('"') {
        Rule::Terminal(raw_rule.as_bytes()[1])
    } else {
        let cases = raw_rule.split(" | ").collect::<Vec<_>>();
        if cases.len() == 1 {
            case_to_rule(parse_case(cases[0]))
        } else {
            let case1 = parse_case(cases[0]);
            let case2 = parse_case(cases[1]);
            if case1.len() < 2 || case2.len() < 2 {
                Rule::Or(vec![case_to_rule(case1), case_to_rule(case2)])
            } else {
                let s1 = case1[0];
                let e1 = case1[1];
                let s2 = case2[0];
                let e2 = case2[1];
                if s1 == s2 {
                    Rule::And(vec![
                        Rule::RuleNum(s1),
                        Rule::Or(vec![Rule::RuleNum(e1), Rule::RuleNum(e2)]),
                    ])
                } else if e1 == e2 {
                    Rule::And(vec![
                        Rule::Or(vec![Rule::RuleNum(s1), Rule::RuleNum(s2)]),
                        Rule::RuleNum(e1),
                    ])
                } else {
                    Rule::Or(vec![
                        Rule::And(vec![Rule::RuleNum(s1), Rule::RuleNum(e1)]),
                        Rule::And(vec![Rule::RuleNum(s2), Rule::RuleNum(e2)]),
                    ])
                }
            }
        }
    };
    (rule_num, rule)
}

fn parse_rules() -> (Vec<Rule>, std::slice::Iter<'static, String>) {
    let mut rules_map = vec![Rule::Terminal(b'X'); INPUT.len()];
    let mut it = INPUT.iter();
    while let Some(line) = it.next() {
        if line.is_empty() {
            break;
        }
        let (rule_num, rule) = parse_rule(&line);
        rules_map[rule_num] = rule;
    }
    (rules_map, it)
}

fn is_valid(rules: &[Rule], rule: &Rule, input: &[u8], idx: usize) -> (usize, bool) {
    match rule {
        &Rule::Terminal(c) => {
            if idx < input.len() {
                (1, input[idx] == c)
            } else {
                (0, false)
            }
        }
        &Rule::RuleNum(n) => is_valid(rules, &rules[n], input, idx),
        Rule::Or(cases) => {
            let mut l: usize = 0;
            let mut ok: bool = false;
            for case in cases {
                (l, ok) = is_valid(rules, case.unwrap(rules), input, idx);
                if ok {
                    break;
                }
            }
            (l, ok)
        }
        Rule::And(cases) => {
            let mut len = 0;
            let mut ok = false;
            for case in cases {
                let l: usize;
                (l, ok) = is_valid(rules, case, input, idx + len);
                if !ok {
                    break;
                }
                len += l;
            }
            (len, ok)
        }
        &Rule::ManyTill(many, till) => {
            let mut len = 0;
            let mut ok: bool;
            loop {
                let (mut l, till_ok) = is_valid(rules, rules[till].unwrap(rules), input, idx + len);
                if till_ok {
                    ok = len != 0;
                    len += l;
                    break;
                };
                (l, ok) = is_valid(rules, rules[many].unwrap(rules), input, idx + len);
                if !ok {
                    break;
                }
                len += l;
            }
            (len, ok)
        }
    }
}

pub fn part1() -> usize {
    let (rules, inputs) = parse_rules();
    inputs
        .filter(|input| {
            let (l, ok) = is_valid(&rules, &rules[0], input.as_bytes(), 0);
            ok && l == input.len()
        })
        .count()
}

pub fn part2() -> usize {
    let (mut rules, inputs) = parse_rules();
    /*
    println!(
        "rules: {}",
        rules
            .iter()
            .filter(|rule| !matches!(rule, Rule::Terminal(b'X')))
            .map(|rule| format!("{}", rule))
            .collect::<Vec<String>>()
            .join("\n")
    );
    */
    rules[0] = Rule::ManyTill(42, 11);
    rules[11] = parse_rule("11: 42 31 | 42 11 31").1;
    inputs
        .filter(|input| {
            let (l, ok) = is_valid(&rules, &rules[0], input.as_bytes(), 0);
            ok && l == input.len()
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn run_part1() {
        assert_eq!(part1(), 111);
    }

    #[test]
    fn run_part2() {
        assert_eq!(part2(), 343);
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
