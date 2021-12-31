use std::fmt;

use ahash::AHashMap;
use shared::utils;

lazy_static! {
    static ref INPUT: Vec<String> = utils::read_input_lines("day24");
}

#[derive(Debug)]
enum Source {
    Reg(usize),
    Imm(isize),
}

#[derive(Debug)]
enum Instruction {
    Input(usize),
    Add(usize, Source),
    Mul(usize, Source),
    Div(usize, Source),
    Mod(usize, Source),
    Eql(usize, Source),
}

fn first_char(s: &str) -> char {
    s.chars().next().unwrap()
}

fn parse_source(s: &str) -> Source {
    let c = first_char(s);
    if c.is_alphabetic() {
        Source::Reg((c as usize) - ('w' as usize))
    } else {
        Source::Imm(s.parse().unwrap())
    }
}

fn parse_lines<I, S>(lines: I) -> Vec<Instruction>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let w = 'w' as usize;
    lines
        .map(|line| {
            let mut parts = line.as_ref().split_whitespace();
            let op = parts.next().unwrap();
            let target = (first_char(parts.next().unwrap()) as usize) - w;
            if op == "inp" {
                return Instruction::Input(target);
            }
            let source = parse_source(parts.next().unwrap());
            match op {
                "add" => Instruction::Add(target, source),
                "mul" => Instruction::Mul(target, source),
                "div" => Instruction::Div(target, source),
                "mod" => Instruction::Mod(target, source),
                "eql" => Instruction::Eql(target, source),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn parse_input() -> Vec<Instruction> {
    parse_lines(INPUT.iter())
}

fn source_value(regs: &[Box<Expr>], s: &Source) -> Box<Expr> {
    match s {
        Source::Reg(reg) => regs[*reg].clone(),
        Source::Imm(v) => Box::new(Expr::Num(*v)),
    }
}

#[derive(Clone)]
enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Eql(Box<Expr>, Box<Expr>),
    NotEql(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Input(usize),
    Num(isize),
    Var(String),
}

#[derive(Clone)]
enum Statement {
    Assign(String, Box<Expr>),
}

fn maybe_paren(e: &Box<Expr>) -> String {
    let ev = e.as_ref();
    match ev {
        Expr::Num(i) => {
            if i < &0 {
                format!("({:?})", ev)
            } else {
                format!("{:?}", ev)
            }
        }
        Expr::Var(_) | Expr::Input(_) => format!("{:?}", ev),
        _ => format!("({:?})", ev),
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Add(lhs, rhs) => write!(f, "{} + {}", maybe_paren(lhs), maybe_paren(rhs)),
            Expr::Mul(lhs, rhs) => write!(f, "{} * {}", maybe_paren(lhs), maybe_paren(rhs)),
            Expr::Div(lhs, rhs) => write!(f, "{} / {}", maybe_paren(lhs), maybe_paren(rhs)),
            Expr::Mod(lhs, rhs) => write!(f, "{} % {}", maybe_paren(lhs), maybe_paren(rhs)),
            Expr::Eql(lhs, rhs) => write!(f, "{} == {}", maybe_paren(lhs), maybe_paren(rhs)),
            Expr::NotEql(lhs, rhs) => write!(f, "{} != {}", maybe_paren(lhs), maybe_paren(rhs)),
            Expr::If(cond, t_branch, f_branch) => write!(
                f,
                "if {:?} {{ {} }} else {{ {} }}",
                cond,
                maybe_paren(t_branch),
                maybe_paren(f_branch)
            ),
            Expr::Input(i) => write!(f, "d({})", i),
            Expr::Num(i) => write!(f, "{}", i),
            Expr::Var(v) => write!(f, "{}", v),
        }
    }
}

impl fmt::Debug for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Assign(lhs, rhs) => write!(f, "{} := {:?}", lhs, rhs),
        }
    }
}

fn pop_reg(regs: &mut [Box<Expr>], i: usize) -> Box<Expr> {
    let mut reg = Box::new(Expr::Num(0));
    std::mem::swap(&mut reg, &mut regs[i]);
    return reg;
}

fn save_to_var(regs: &mut [Box<Expr>], v: &str, reg: usize) -> Statement {
    let e = pop_reg(regs, reg);
    let s = Statement::Assign(v.to_string(), simplify_expr(e));
    regs[reg] = Box::new(Expr::Var(v.to_string()));
    s
}

fn run(insts: &[Instruction]) -> Vec<Statement> {
    let mut next_input = 0;
    let mut regs = [
        Box::new(Expr::Num(0)),
        Box::new(Expr::Num(0)),
        Box::new(Expr::Num(0)),
        Box::new(Expr::Num(0)),
    ];
    let mut statements = vec![];
    for inst in insts.iter() {
        match inst {
            Instruction::Input(reg) => {
                // let stmt = save_to_var(&mut regs, "val", 3);
                // statements.push(stmt);
                let e = simplify_expr(pop_reg(&mut regs, 3));
                regs[3] = e.clone();
                statements.push(Statement::Assign("val".to_string(), e));

                let v = Box::new(Expr::Input(next_input));
                next_input += 1;
                regs[*reg] = v;
            }
            Instruction::Add(target, source) => {
                let lhs = pop_reg(&mut regs, *target);
                regs[*target] = Box::new(Expr::Add(lhs, source_value(&regs, source)))
            }
            Instruction::Mul(target, source) => {
                let lhs = pop_reg(&mut regs, *target);
                regs[*target] = Box::new(Expr::Mul(lhs, source_value(&regs, source)))
            }
            Instruction::Div(target, source) => {
                let lhs = pop_reg(&mut regs, *target);
                regs[*target] = Box::new(Expr::Div(lhs, source_value(&regs, source)))
            }
            Instruction::Mod(target, source) => {
                let lhs = pop_reg(&mut regs, *target);
                regs[*target] = Box::new(Expr::Mod(lhs, source_value(&regs, source)))
            }
            Instruction::Eql(target, source) => {
                /*
                let lhs = if matches!(&*regs[*target], &Expr::Eql(_, _)) {
                    pop_reg(&mut regs, *target)
                } else {
                    let stmt = save_to_var(&mut regs, "tmp", *target);
                    statements.push(stmt);
                    Box::new(Expr::Var("tmp".to_string()))
                };
                */
                let lhs = pop_reg(&mut regs, *target);
                regs[*target] = Box::new(Expr::Eql(lhs, source_value(&regs, source)));
            }
        }
    }
    let z = pop_reg(&mut regs, 3);
    statements.push(Statement::Assign("val".to_string(), simplify_expr(z)));
    regs[3] = Box::new(Expr::Var("val".to_string()));
    statements
}

fn min(e: &Box<Expr>) -> Option<isize> {
    match e.as_ref() {
        Expr::Add(lhs, rhs) => {
            let x = min(lhs)?;
            let y = min(rhs)?;
            Some(x + y)
        }
        Expr::Mul(lhs, rhs) => Some(min(lhs)? * min(rhs)?),
        Expr::Div(lhs, rhs) => Some(min(lhs)? / max(rhs)?),
        Expr::Mod(_, _) => Some(0),
        Expr::Eql(_, _) => Some(0),
        Expr::NotEql(_, _) => Some(0),
        Expr::If(_, t_branch, f_branch) => Some(min(t_branch)?.min(min(f_branch)?)),
        Expr::Input(_) => Some(1),
        Expr::Num(n) => Some(*n),
        Expr::Var(_) => None,
    }
}

fn max(e: &Box<Expr>) -> Option<isize> {
    match e.as_ref() {
        Expr::Add(lhs, rhs) => Some(max(lhs)? + max(rhs)?),
        Expr::Mul(lhs, rhs) => Some(max(lhs)? * max(rhs)?),
        Expr::Div(lhs, rhs) => Some(max(lhs)? / min(rhs)?),
        Expr::Mod(_, rhs) => Some(max(rhs)? - 1),
        Expr::Eql(_, _) => Some(0),
        Expr::NotEql(_, _) => Some(0),
        Expr::If(_, t_branch, f_branch) => Some(max(t_branch)?.max(max(f_branch)?)),
        Expr::Input(_) => Some(1),
        Expr::Num(n) => Some(*n),
        Expr::Var(_) => None,
    }
}

fn simplify_expr(e: Box<Expr>) -> Box<Expr> {
    match *e {
        Expr::Add(lhs, rhs) => {
            let new_lhs = simplify_expr(lhs);
            let new_rhs = simplify_expr(rhs);
            match (*new_lhs, *new_rhs) {
                (Expr::Num(n1), Expr::Num(n2)) => Box::new(Expr::Num(n1 + n2)),
                (new_lhs @ Expr::Num(_), new_rhs) => {
                    simplify_expr(Box::new(Expr::Add(Box::new(new_rhs), Box::new(new_lhs))))
                }
                (new_rhs, Expr::Num(0)) => Box::new(new_rhs),
                (Expr::Add(add_lhs, box Expr::Num(n1)), Expr::Num(n2)) => {
                    Box::new(Expr::Add(add_lhs, Box::new(Expr::Num(n1 + n2))))
                }
                (Expr::Add(add_lhs, n @ box Expr::Num(_)), other) => simplify_expr(Box::new(
                    Expr::Add(Box::new(Expr::Add(add_lhs, Box::new(other))), n),
                )),
                (lhs, Expr::Add(a2_lhs, a2_rhs)) => {
                    let new_lhs = simplify_expr(Box::new(Expr::Add(Box::new(lhs), a2_lhs)));
                    simplify_expr(Box::new(Expr::Add(new_lhs, a2_rhs)))
                }
                (new_lhs, new_rhs) => Box::new(Expr::Add(Box::new(new_lhs), Box::new(new_rhs))),
            }
        }
        Expr::Mul(lhs, rhs) => {
            let new_lhs = simplify_expr(lhs);
            let new_rhs = simplify_expr(rhs);
            match (*new_lhs, *new_rhs) {
                (Expr::Num(0), _) | (_, Expr::Num(0)) => Box::new(Expr::Num(0)),
                (Expr::Num(1), other) | (other, Expr::Num(1)) => Box::new(other),
                (Expr::Num(n1), Expr::Num(n2)) => Box::new(Expr::Num(n1 * n2)),
                (n @ Expr::Num(_), other) => Box::new(Expr::Mul(Box::new(other), Box::new(n))),
                (Expr::Add(add_lhs, add_rhs), m) | (m, Expr::Add(add_lhs, add_rhs)) => {
                    let new_lhs = Box::new(Expr::Mul(Box::new(m.clone()), add_lhs));
                    let new_rhs = Box::new(Expr::Mul(Box::new(m), add_rhs));
                    simplify_expr(Box::new(Expr::Add(
                        simplify_expr(new_lhs),
                        simplify_expr(new_rhs),
                    )))
                }
                (Expr::Mul(m_lhs, box Expr::Num(n1)), Expr::Num(n2)) => {
                    Box::new(Expr::Mul(m_lhs, Box::new(Expr::Num(n1 * n2))))
                }
                (lhs, Expr::Mul(m_lhs, m_rhs)) => {
                    let new_lhs = simplify_expr(Box::new(Expr::Mul(Box::new(lhs), m_lhs)));
                    simplify_expr(Box::new(Expr::Add(new_lhs, m_rhs)))
                }
                /*
                (other, cond @ Expr::Eql(_, _))
                | (cond @ Expr::Eql(_, _), other)
                | (other, cond @ Expr::NotEql(_, _))
                | (cond @ Expr::NotEql(_, _), other) => Box::new(Expr::If(
                    Box::new(cond),
                    Box::new(other),
                    Box::new(Expr::Num(0)),
                )),
                */
                (new_lhs, new_rhs) => Box::new(Expr::Mul(Box::new(new_lhs), Box::new(new_rhs))),
            }
        }
        Expr::Div(lhs, rhs) => {
            let new_lhs = simplify_expr(lhs);
            let new_rhs = simplify_expr(rhs);
            match (*new_lhs, *new_rhs) {
                (Expr::Num(0), _) => Box::new(Expr::Num(0)),
                (new_lhs, Expr::Num(1)) => Box::new(new_lhs),
                (Expr::Add(add_lhs, add_rhs), m) | (m, Expr::Add(add_lhs, add_rhs)) => {
                    let new_lhs = Box::new(Expr::Mul(Box::new(m.clone()), add_lhs));
                    let new_rhs = Box::new(Expr::Mul(Box::new(m), add_rhs));
                    simplify_expr(Box::new(Expr::Add(
                        simplify_expr(new_lhs),
                        simplify_expr(new_rhs),
                    )))
                }
                (new_lhs, new_rhs) => Box::new(Expr::Div(Box::new(new_lhs), Box::new(new_rhs))),
            }
        }
        Expr::Mod(lhs, rhs) => {
            let new_lhs = simplify_expr(lhs);
            let new_rhs = simplify_expr(rhs);
            if max(&new_lhs) < min(&new_rhs) {
                return new_lhs;
            }
            match (*new_lhs, *new_rhs) {
                (Expr::Num(0), _) => Box::new(Expr::Num(0)),
                (Expr::Num(n1), Expr::Num(n2)) => Box::new(Expr::Num(n1 % n2)),
                (i @ Expr::Input(_), Expr::Num(n)) => {
                    if n >= 10 {
                        Box::new(i)
                    } else {
                        Box::new(Expr::Mod(Box::new(i), Box::new(Expr::Num(n))))
                    }
                }
                (lhs @ Expr::Eql(_, _), _) | (lhs @ Expr::NotEql(_, _), _) => Box::new(lhs),
                (Expr::Mod(other, box Expr::Num(n1)), Expr::Num(n2)) => {
                    let new_rhs = Box::new(Expr::Num(n2));
                    if n1 == n2 {
                        return Box::new(Expr::Mod(other, new_rhs));
                    }
                    return Box::new(Expr::Mod(
                        Box::new(Expr::Mod(other, Box::new(Expr::Num(n1)))),
                        new_rhs,
                    ));
                }
                (Expr::Mul(m_lhs, m_rhs), n @ Expr::Num(_)) => {
                    let n_expr = Box::new(n);
                    let new_expr = simplify_expr(Box::new(Expr::Mul(
                        Box::new(Expr::Mod(m_lhs, n_expr.clone())),
                        Box::new(Expr::Mod(m_rhs, n_expr.clone())),
                    )));
                    if matches!(&*new_expr, Expr::Num(0)) {
                        return Box::new(Expr::Num(0));
                    }
                    return Box::new(Expr::Mod(new_expr, n_expr.clone()));
                }
                (Expr::Add(add_lhs, add_rhs), n @ Expr::Num(_)) => {
                    let n_expr = Box::new(n);
                    let new_expr = simplify_expr(Box::new(Expr::Add(
                        Box::new(Expr::Mod(add_lhs, n_expr.clone())),
                        Box::new(Expr::Mod(add_rhs, n_expr.clone())),
                    )));
                    if matches!(&*new_expr, Expr::Num(0)) {
                        return Box::new(Expr::Num(0));
                    }
                    return Box::new(Expr::Mod(new_expr, n_expr.clone()));
                }
                (new_lhs, new_rhs) => Box::new(Expr::Mod(Box::new(new_lhs), Box::new(new_rhs))),
            }
        }
        Expr::Eql(lhs, rhs) => {
            let new_lhs = simplify_expr(lhs);
            let new_rhs = simplify_expr(rhs);
            if min(&new_lhs) > max(&new_rhs) || max(&new_lhs) < min(&new_rhs) {
                return Box::new(Expr::Num(0));
            }
            match (&*new_lhs, &*new_rhs) {
                (Expr::Eql(_, _), Expr::Num(0)) => Box::new(Expr::NotEql(new_lhs, new_rhs)),
                (Expr::Num(n), Expr::Input(_)) => {
                    if n >= &10 || n <= &1 {
                        Box::new(Expr::Num(0))
                    } else {
                        Box::new(Expr::Eql(new_lhs, new_rhs))
                    }
                }
                (Expr::Num(n1), Expr::Num(n2)) => {
                    if n1 == n2 {
                        Box::new(Expr::Num(1))
                    } else {
                        Box::new(Expr::Num(0))
                    }
                }
                _ => Box::new(Expr::Eql(new_lhs, new_rhs)),
            }
        }
        Expr::NotEql(lhs, rhs) => {
            let new_lhs = simplify_expr(lhs);
            let new_rhs = simplify_expr(rhs);
            if matches!(&*new_rhs, Expr::Num(0)) {
                return new_lhs;
            }
            Box::new(Expr::NotEql(new_lhs, new_rhs))
        }
        Expr::If(cond, t_branch, f_branch) => Box::new(Expr::If(
            simplify_expr(cond),
            simplify_expr(t_branch),
            simplify_expr(f_branch),
        )),
        Expr::Input(_) | Expr::Num(_) | Expr::Var(_) => e,
    }
}

fn from_digits(digits: &[usize]) -> usize {
    let mut ret = 0;
    for d in digits.iter() {
        ret = (ret * 10) + d;
    }
    ret
}

fn evaluate(e: &Box<Expr>, val: isize, tmp: isize, digit: usize) -> Result<isize, String> {
    let res = match e.as_ref() {
        Expr::Add(lhs, rhs) => {
            let lhs = evaluate(lhs, val, tmp, digit)?;
            let rhs = evaluate(rhs, val, tmp, digit)?;
            lhs + rhs
        }
        Expr::Mul(lhs, rhs) => {
            let lhs = evaluate(lhs, val, tmp, digit)?;
            let rhs = evaluate(rhs, val, tmp, digit)?;
            lhs * rhs
        }
        Expr::Div(lhs, rhs) => {
            let lhs = evaluate(lhs, val, tmp, digit)?;
            let rhs = evaluate(rhs, val, tmp, digit)?;
            if rhs == 0 {
                return Err("bad div".to_string());
            }
            lhs / rhs
        }
        Expr::Mod(lhs, rhs) => {
            let lhs = evaluate(lhs, val, tmp, digit)?;
            let rhs = evaluate(rhs, val, tmp, digit)?;
            if lhs < 0 || rhs <= 0 {
                return Err("bad mod".to_string());
            }
            lhs % rhs
        }
        Expr::Eql(lhs, rhs) => {
            let lhs = evaluate(lhs, val, tmp, digit)?;
            let rhs = evaluate(rhs, val, tmp, digit)?;
            if lhs == rhs {
                1
            } else {
                0
            }
        }
        Expr::NotEql(lhs, rhs) => {
            let lhs = evaluate(lhs, val, tmp, digit)?;
            let rhs = evaluate(rhs, val, tmp, digit)?;
            if lhs != rhs {
                1
            } else {
                0
            }
        }
        Expr::If(cond, t_branch, f_branch) => {
            let cond = evaluate(cond, val, tmp, digit)?;
            return if cond == 1 {
                evaluate(t_branch, val, tmp, digit)
            } else {
                evaluate(f_branch, val, tmp, digit)
            };
        }
        Expr::Input(_) => digit as isize,
        Expr::Num(n) => *n,
        Expr::Var(s) => {
            if s == "tmp" {
                tmp
            } else if s == "val" {
                val
            } else {
                return Err(format!("undefined variable: {}", s));
            }
        }
    };
    Ok(res)
}

#[derive(Debug)]
struct Output {
    digit: usize,
    output: isize,
}

fn evaluate_all_digits(stmt: &Statement, val: isize, tmp: isize) -> Vec<Output> {
    let Statement::Assign(_, e) = stmt;
    let mut options = vec![];
    for digit in 1..10 {
        if let Ok(res) = evaluate(e, val, tmp, digit) {
            options.push(Output {
                digit: digit as usize,
                output: res,
            });
        }
    }
    options
}

fn find_model_number(
    memo: &mut AHashMap<(usize, isize), Option<Vec<usize>>>,
    stmts: &[Statement],
    i: usize,
    input: isize,
    tmp: isize,
    biggest: bool,
) -> Option<Vec<usize>> {
    let key = (i, input);
    if let Some(v) = memo.get(&key) {
        return v.clone();
    }

    if i == stmts.len() {
        let ret = if input == 0 { Some(vec![]) } else { None };
        memo.insert(key, ret.clone());
        return ret;
    }

    let Statement::Assign(s, e) = &stmts[i];
    if s == "tmp" {
        let tmp = evaluate(e, input, tmp, 0).unwrap();
        let ret = find_model_number(memo, stmts, i + 1, input, tmp, biggest);
        memo.insert(key, ret.clone());
        return ret;
    }

    let mut options = evaluate_all_digits(&stmts[i], input, tmp);
    options.sort_by_key(|o| o.digit);
    if biggest {
        options.reverse();
    }

    let mut ret = None;
    for opt in options {
        if let Some(mut res) = find_model_number(memo, stmts, i + 1, opt.output, tmp, biggest) {
            res.insert(0, opt.digit);
            ret = Some(res);
            break;
        }
    }
    memo.insert(key, ret.clone());
    ret
}

fn solve(biggest: bool) -> usize {
    let insts = parse_input();
    let stmts = run(&insts);
    for (i, stmt) in stmts.iter().enumerate() {
        println!("{:02}: {:?}", i, stmt);
    }
    let solution = find_model_number(&mut AHashMap::new(), &stmts, 1, 0, 0, biggest);
    from_digits(&solution.unwrap()[..])
    // 0
}

pub fn part1() -> usize {
    solve(true)
}

pub fn part2() -> usize {
    solve(false)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_mod_to_zero() {}
}
