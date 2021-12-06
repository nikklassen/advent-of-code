use shared::utils;

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day03");
    static ref INPUT: Vec<String> = utils::read_input_lines("day03");
}

fn parse_input() -> Vec<Vec<char>> {
    INPUT.iter().map(|line| line.chars().collect()).collect()
}

pub fn part1() -> usize {
    let nums = parse_input();
    let n_bits = nums[0].len();
    let gamma_bits = nums.iter().fold(vec![0; n_bits], |mut acc, n| {
        for (i, c) in n.iter().enumerate() {
            if *c == '1' {
                acc[i] += 1
            } else {
                acc[i] -= 1
            }
        }
        acc
    });
    let gamma_str: String = gamma_bits
        .iter()
        .map(|b| if *b >= 0 { '1' } else { '0' })
        .collect();
    let gamma = usize::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = !gamma & ((1 << n_bits) - 1);
    gamma * epsilon
}

fn most_common_bit(nums: &[Vec<char>], bit: usize) -> char {
    let c = nums
        .iter()
        .fold(0, |acc, n| if n[bit] == '1' { acc + 1 } else { acc - 1 });
    if c >= 0 {
        '1'
    } else {
        '0'
    }
}

fn find_rating<F>(mut nums: Vec<Vec<char>>, f: F) -> usize
where
    F: Fn(char, char) -> bool,
{
    let mut bit = 0;
    while nums.len() > 1 {
        let mcb = most_common_bit(&nums, bit);
        nums = nums.into_iter().filter(|num| f(num[bit], mcb)).collect();
        bit += 1;
    }
    let rating_str: String = nums.first().unwrap().iter().cloned().collect();
    usize::from_str_radix(&rating_str, 2).unwrap()
}

fn find_o2_rating(nums: Vec<Vec<char>>) -> usize {
    find_rating(nums, |num_bit, mcb| num_bit == mcb)
}

fn find_co2_rating(nums: Vec<Vec<char>>) -> usize {
    find_rating(nums, |num_bit, mcb| num_bit != mcb)
}

pub fn part2() -> usize {
    let nums = parse_input();
    let o2_rating = find_o2_rating(nums.clone());
    let co2_rating = find_co2_rating(nums.clone());
    o2_rating * co2_rating
}
