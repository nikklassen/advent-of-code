use itertools::Itertools;

use crate::utils::{self, *};

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day21");
    static ref INPUT: Vec<String> = utils::read_input_lines("day21");
}

type Ingredient = usize;

type Allergen = usize;

#[derive(Debug, Default)]
struct Food {
    ingredients: Vec<Ingredient>,
    allergens: Vec<Allergen>,
}

fn parse_input() -> (
    Vec<Food>,
    AHashMap<&'static str, Ingredient>,
    AHashMap<&'static str, Allergen>,
) {
    let mut ingredients = AHashMap::with_capacity(INPUT.len());
    let mut allergens = AHashMap::with_capacity(INPUT.len());

    let mut next_ingredient = 0;
    let mut next_allergen = 0;

    let foods = INPUT
        .iter()
        .map(|line| {
            let mut f = Food {
                ingredients: vec![],
                allergens: vec![],
            };
            let mut is_allergens = false;
            for part in line.split_whitespace() {
                if part == "(contains" {
                    is_allergens = true;
                    continue;
                }
                if !is_allergens {
                    let i = part;
                    let i_v = ingredients.entry(i).or_insert_with(|| {
                        let v = next_ingredient;
                        next_ingredient += 1;
                        v
                    });
                    f.ingredients.push(*i_v);
                } else {
                    let a = &part[0..part.len() - 1];
                    let a_v = allergens.entry(a).or_insert_with(|| {
                        let v = next_allergen;
                        next_allergen += 1;
                        v
                    });
                    f.allergens.push(*a_v);
                }
            }
            f
        })
        .collect_vec();
    (foods, ingredients, allergens)
}

#[derive(Debug)]
struct AllergenOptionList {
    allergen: Allergen,
    options: AHashSet<Ingredient>,
}

fn next_ingredient_and_options(
    all_ingredients: &mut Vec<Ingredient>,
    allergen_options: &mut AHashMap<Allergen, AllergenOptionList>,
) -> Option<(Ingredient, Vec<Option<Allergen>>)> {
    all_ingredients.pop().map(|i| {
        let mut allergens = vec![];
        for (allergen, option_list) in allergen_options.iter() {
            if option_list.options.contains(&i) {
                allergens.push(Some(*allergen));
            }
        }
        allergens.push(None);
        (i, allergens)
    })
}

fn solve(
    all_ingredients: &mut Vec<Ingredient>,
    allergen_options: &mut AHashMap<Allergen, AllergenOptionList>,
    assigned_ingredients: &mut Vec<Option<Allergen>>,
) -> bool {
    let next_options = next_ingredient_and_options(all_ingredients, allergen_options);

    if next_options.is_none() {
        return allergen_options.len() == 0;
    }

    let (i, next_allergens) = next_options.unwrap();

    let mut ret = false;
    for allergen in next_allergens.into_iter() {
        assigned_ingredients[i] = allergen;
        let mut allergen_option: Option<AllergenOptionList> = None;
        if let Some(a) = allergen {
            allergen_option = allergen_options.remove(&a);
        }

        let solved = solve(all_ingredients, allergen_options, assigned_ingredients);

        if let Some(a) = allergen {
            allergen_options.insert(a, allergen_option.unwrap());
        }
        if solved {
            ret = solved;
            break;
        }

        assigned_ingredients[i] = None;
    }

    all_ingredients.push(i);
    ret
}

fn map_allergens(
    foods: &Vec<Food>,
    ingredient_mapping: &AHashMap<&'static str, Ingredient>,
    allergen_mapping: &AHashMap<&'static str, Allergen>,
) -> Vec<Option<Ingredient>> {
    let mut all_ingredients: Vec<Ingredient> = (0..ingredient_mapping.len()).collect();
    let mut allergen_options: AHashMap<Allergen, AllergenOptionList> =
        AHashMap::with_capacity(allergen_mapping.len());
    for food in foods.iter() {
        for allergen in food.allergens.iter() {
            if !allergen_options.contains_key(allergen) {
                allergen_options.insert(
                    *allergen,
                    AllergenOptionList {
                        allergen: *allergen,
                        options: food.ingredients.iter().cloned().collect(),
                    },
                );
                continue;
            }
            let option_list = allergen_options.get_mut(&allergen).unwrap();
            let mut new_options = AHashSet::with_capacity(option_list.options.len());
            for i in food.ingredients.iter() {
                if option_list.options.contains(i) {
                    new_options.insert(*i);
                }
            }
            option_list.options = new_options;
        }
    }

    let num_ingredients = all_ingredients.len();
    let mut solution = vec![None; num_ingredients];
    if !solve(&mut all_ingredients, &mut allergen_options, &mut solution) {
        panic!("couldn't find a solution");
    }

    solution
}

pub fn part1() -> usize {
    let (foods, ingredient_mapping, allergen_mapping) = parse_input();

    let solution = map_allergens(&foods, &ingredient_mapping, &allergen_mapping);

    let mut no_allergens = AHashSet::with_capacity(solution.len());
    for (i, allergen) in solution.iter().enumerate() {
        if allergen.is_none() {
            no_allergens.insert(i);
        }
    }

    let mut c = 0;
    for food in foods {
        for ingredient in food.ingredients.iter() {
            if no_allergens.contains(ingredient) {
                c += 1;
            }
        }
    }
    c
}

pub fn part2() -> String {
    let (foods, ingredient_mapping, allergen_mapping) = parse_input();

    let solution = map_allergens(&foods, &ingredient_mapping, &allergen_mapping);

    let mut rev_ingredient_mapping = AHashMap::with_capacity(ingredient_mapping.len());
    for (name, number) in ingredient_mapping.iter() {
        rev_ingredient_mapping.insert(number, name);
    }
    let mut rev_allergen_mapping = AHashMap::with_capacity(allergen_mapping.len());
    for (name, number) in allergen_mapping.iter() {
        rev_allergen_mapping.insert(number, name);
    }

    let mut dangerous_ingredients = solution
        .iter()
        .enumerate()
        .filter_map(|(ingredient, allergen)| {
            if let Some(a) = allergen {
                Some((
                    *rev_ingredient_mapping[&ingredient],
                    *rev_allergen_mapping[&a],
                ))
            } else {
                None
            }
        })
        .collect_vec();
    dangerous_ingredients.sort_by_key(|&(_, a)| a);
    dangerous_ingredients.iter().map(|(i, _)| i).join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn run_part1() {
        assert_eq!(part1(), 2203);
    }

    #[test]
    fn run_part2() {
        assert_eq!(part2(), "fqfm,kxjttzg,ldm,mnzbc,zjmdst,ndvrq,fkjmz,kjkrm");
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
