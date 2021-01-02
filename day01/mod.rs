use crate::utils;

fn read_vals() -> Vec<i32> {
    let input = utils::read_input_lines("day01");
    input
        .iter()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<i32>>()
}

fn two_sum_map(vals: &[i32]) -> Vec<Vec<i32>> {
    (0..vals.len())
        .map(|i| {
            (0..vals.len())
                .map(|j| if j > i { vals[i] + vals[j] } else { 0 })
                .collect()
        })
        .collect()
}

pub fn part1() -> i32 {
    let vals = read_vals();
    let m = two_sum_map(&vals);
    m.iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter().enumerate().find_map(|(j, &v)| {
                if v == 2020 {
                    Some(vals[i] * vals[j])
                } else {
                    None
                }
            })
        })
        .unwrap()
}

pub fn part2() -> i32 {
    let vals = read_vals();
    let m = two_sum_map(&vals);
    for k in 2..vals.len() {
        if let Some(v) = m.iter().enumerate().find_map(|(i, row)| {
            row.iter().enumerate().find_map(|(j, &v)| {
                if v > 0 && v + vals[k] == 2020 {
                    Some(vals[i] * vals[j] * vals[k])
                } else {
                    None
                }
            })
        }) {
            return v;
        }
    }
    unreachable!()
}
