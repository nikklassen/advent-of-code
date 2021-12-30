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
                let stmt = save_to_var(&mut regs, "val", 3);
                statements.push(stmt);

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

fn simplify_expr(e: Box<Expr>) -> Box<Expr> {
    match *e {
        Expr::Add(lhs, rhs) => {
            let new_lhs = simplify_expr(lhs);
            let new_rhs = simplify_expr(rhs);
            if let Expr::Num(0) = *new_lhs {
                return new_rhs;
            }
            if let Expr::Num(0) = *new_rhs {
                return new_lhs;
            }
            return Box::new(Expr::Add(new_lhs, new_rhs));
        }
        Expr::Mul(lhs, rhs) => {
            let new_lhs = simplify_expr(lhs);
            let new_rhs = simplify_expr(rhs);
            match (*new_lhs, *new_rhs) {
                (Expr::Num(0), _) | (_, Expr::Num(0)) => Box::new(Expr::Num(0)),
                (Expr::Num(1), other) | (other, Expr::Num(1)) => Box::new(other),
                (other, cond @ Expr::Eql(_, _))
                | (cond @ Expr::Eql(_, _), other)
                | (other, cond @ Expr::NotEql(_, _))
                | (cond @ Expr::NotEql(_, _), other) => Box::new(Expr::If(
                    Box::new(cond),
                    Box::new(other),
                    Box::new(Expr::Num(0)),
                )),
                (new_lhs, new_rhs) => Box::new(Expr::Mul(Box::new(new_lhs), Box::new(new_rhs))),
            }
        }
        Expr::Div(lhs, rhs) => {
            let new_lhs = simplify_expr(lhs);
            let new_rhs = simplify_expr(rhs);
            if let Expr::Num(0) = *new_lhs {
                return Box::new(Expr::Num(0));
            }
            if let Expr::Num(1) = *new_rhs {
                return new_lhs;
            }
            return Box::new(Expr::Div(new_lhs, new_rhs));
        }
        Expr::Mod(lhs, rhs) => {
            let new_lhs = simplify_expr(lhs);
            let new_rhs = simplify_expr(rhs);
            if let Expr::Num(0) = *new_lhs {
                return Box::new(Expr::Num(0));
            }
            return Box::new(Expr::Mod(new_lhs, new_rhs));
        }
        Expr::Eql(lhs, rhs) => {
            let new_lhs = simplify_expr(lhs);
            let new_rhs = simplify_expr(rhs);
            match (*new_lhs, *new_rhs) {
                (Expr::Eql(sub_lhs, sub_rhs), Expr::Num(0)) => {
                    Box::new(Expr::NotEql(sub_lhs, sub_rhs))
                }
                (new_lhs, new_rhs) => Box::new(Expr::Eql(Box::new(new_lhs), Box::new(new_rhs))),
            }
        }
        Expr::NotEql(lhs, rhs) => {
            let new_lhs = simplify_expr(lhs);
            let new_rhs = simplify_expr(rhs);
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

fn evaluate(e: &Box<Expr>, val: isize, digit: usize) -> Result<isize, String> {
    let res = match e.as_ref() {
        Expr::Add(lhs, rhs) => {
            let lhs = evaluate(lhs, val, digit)?;
            let rhs = evaluate(rhs, val, digit)?;
            lhs + rhs
        }
        Expr::Mul(lhs, rhs) => {
            let lhs = evaluate(lhs, val, digit)?;
            let rhs = evaluate(rhs, val, digit)?;
            lhs * rhs
        }
        Expr::Div(lhs, rhs) => {
            let lhs = evaluate(lhs, val, digit)?;
            let rhs = evaluate(rhs, val, digit)?;
            if rhs == 0 {
                return Err("bad div".to_string());
            }
            lhs / rhs
        }
        Expr::Mod(lhs, rhs) => {
            let lhs = evaluate(lhs, val, digit)?;
            let rhs = evaluate(rhs, val, digit)?;
            if lhs < 0 || rhs <= 0 {
                return Err("bad mod".to_string());
            }
            lhs % rhs
        }
        Expr::Eql(lhs, rhs) => {
            let lhs = evaluate(lhs, val, digit)?;
            let rhs = evaluate(rhs, val, digit)?;
            if lhs == rhs {
                1
            } else {
                0
            }
        }
        Expr::NotEql(lhs, rhs) => {
            let lhs = evaluate(lhs, val, digit)?;
            let rhs = evaluate(rhs, val, digit)?;
            if lhs != rhs {
                1
            } else {
                0
            }
        }
        Expr::If(cond, t_branch, f_branch) => {
            let cond = evaluate(cond, val, digit)?;
            return if cond == 1 {
                evaluate(t_branch, val, digit)
            } else {
                evaluate(f_branch, val, digit)
            };
        }
        Expr::Input(_) => digit as isize,
        Expr::Num(n) => *n,
        Expr::Var(_) => val,
    };
    Ok(res)
}

#[derive(Debug)]
struct Output {
    digit: usize,
    output: isize,
}

fn evaluate_all_digits(stmt: &Statement, val: isize) -> Vec<Output> {
    let Statement::Assign(_, e) = stmt;
    let mut options = vec![];
    for digit in 1..10 {
        if let Ok(res) = evaluate(e, val, digit) {
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

    let mut options = evaluate_all_digits(&stmts[i], input);
    options.sort_by_key(|o| o.digit);
    if biggest {
        options.reverse();
    }

    let mut ret = None;
    for opt in options {
        if let Some(mut res) = find_model_number(memo, stmts, i + 1, opt.output, biggest) {
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
    /*
    for (i, stmt) in stmts.iter().enumerate() {
        println!("{:02}: {:?}", i, stmt);
    }
    */
    let solution = find_model_number(&mut AHashMap::new(), &stmts, 1, 0, biggest);
    from_digits(&solution.unwrap()[..])
}

pub fn part1() -> usize {
    solve(true)
}

pub fn part2() -> usize {
    // solve(false)
    0
}
