use crate::utils;

#[derive(Debug)]
enum Expr {
    Num(usize),
    Add,
    Mul,
    Paren(Vec<Expr>),
}

pub fn read_input() -> Vec<String> {
    utils::read_input_lines("day18")
}

fn parse(line: &str) -> Vec<Expr> {
    let mut stack = vec![vec![]];
    for s in line.split_whitespace() {
        let t = match s {
            "*" => Expr::Mul,
            "+" => Expr::Add,
            _ => {
                let mut inner = s;
                while let Some(d) = inner.strip_prefix('(') {
                    stack.push(vec![]);
                    inner = d
                }
                let mut trailing = 0;
                while let Some(d) = inner.strip_suffix(')') {
                    trailing += 1;
                    inner = d;
                }
                stack
                    .last_mut()
                    .unwrap()
                    .push(Expr::Num(inner.parse().unwrap()));
                for _ in 0..trailing {
                    let last = stack.pop().unwrap();
                    stack.last_mut().unwrap().push(Expr::Paren(last));
                }
                continue;
            }
        };
        stack.last_mut().unwrap().push(t);
    }
    stack.pop().unwrap()
}

fn eval_expr(exprs: &[Expr]) -> usize {
    let mut prev_op = None;
    let mut tot = 0;
    for mut expr in exprs {
        let new_expr: Expr;
        if let Expr::Paren(inner) = expr {
            new_expr = Expr::Num(eval_expr(inner));
            expr = &new_expr;
        }
        match expr {
            Expr::Num(n) => match prev_op {
                Some(Expr::Add) => tot += n,
                Some(Expr::Mul) => tot *= n,
                None => tot = *n,
                _ => unreachable!(),
            },
            Expr::Add => prev_op = Some(Expr::Add),
            Expr::Mul => prev_op = Some(Expr::Mul),
            Expr::Paren(_) => unreachable!(),
        }
    }
    tot
}

fn group_by_precendence(exprs: Vec<Expr>) -> Vec<Expr> {
    fn group_to_expr(mut group: Vec<Expr>) -> Expr {
        if group.len() == 1 {
            group.pop().unwrap()
        } else {
            Expr::Paren(group)
        }
    }

    let mut grouped_exprs = vec![];
    let mut group = vec![];
    for expr in exprs {
        match expr {
            Expr::Paren(inner) => {
                group.push(group_to_expr(group_by_precendence(inner)));
            }
            Expr::Num(_) | Expr::Add => {
                group.push(expr);
            }
            Expr::Mul => {
                grouped_exprs.push(group_to_expr(group));
                grouped_exprs.push(Expr::Mul);
                group = vec![];
            }
        }
    }
    if !group.is_empty() {
        grouped_exprs.push(group_to_expr(group));
    }
    grouped_exprs
}

fn solve_with_precendence<F: Fn(Vec<Expr>) -> Vec<Expr>>(group_by_fn: F) -> usize {
    let input = read_input();
    let mut tot = 0;
    for line in input {
        let tokens = parse(&line);
        tot += eval_expr(&group_by_fn(tokens));
    }
    tot
}

pub fn part1() -> usize {
    solve_with_precendence(|g| g)
}

pub fn part2() -> usize {
    solve_with_precendence(group_by_precendence)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part2);
    }
}
