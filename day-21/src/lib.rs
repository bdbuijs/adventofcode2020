use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use nom::{
    bytes::complete::tag,
    character::complete::char as nomchar,
    character::complete::{alpha1, newline, space1},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

pub fn process_part1(input: &str) -> String {
    let (_, parse_result) = parse_input(input).unwrap();
    let (_, allergens) = find_allergens(&parse_result);
    parse_result
        .into_iter()
        .flat_map(|(foods, _)| foods.into_iter())
        .filter(|&food| !allergens.contains(food))
        .count()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, parse_result) = parse_input(input).unwrap();
    let (allergen_map, _) = find_allergens(&parse_result);
    let mut list: Vec<_> = allergen_map
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().next().unwrap()))
        .collect();
    list.sort_by(|a, b| a.0.cmp(b.0));
    let mut answer = String::new();
    Itertools::intersperse(list.into_iter().map(|(_allergen, food)| food), ",")
        .for_each(|s| answer.push_str(s));
    answer
}

fn find_allergens<'a>(
    parse_result: &[(Vec<&'a str>, Vec<&'a str>)],
) -> (HashMap<&'a str, HashSet<&'a str>>, HashSet<&'a str>) {
    let allergen_sets: Vec<_> = parse_result
        .iter()
        .flat_map(|(foods, allergens)| {
            let set = HashSet::<_>::from_iter(foods.iter().cloned());
            allergens
                .iter()
                .map(move |&allergen| (allergen, set.clone()))
        })
        .collect();
    let mut allergen_map = HashMap::new();
    allergen_sets.into_iter().for_each(|(allergen, foods)| {
        allergen_map
            .entry(allergen)
            .or_insert_with(Vec::new)
            .push(foods);
    });
    let mut allergen_map: HashMap<&str, HashSet<&str>> = allergen_map
        .into_iter()
        .map(|(k, v)| {
            let first = v.first().unwrap().clone();
            (
                k,
                v.into_iter()
                    .fold(first, |acc, el| acc.intersection(&el).copied().collect()),
            )
        })
        .collect();

    let mut done = false;
    let mut singles = HashSet::new();
    while !done {
        done = true;
        allergen_map
            .iter()
            .filter(|(_k, v)| v.len() == 1)
            .flat_map(|(_k, v)| v.iter().copied())
            .for_each(|food| {
                singles.insert(food);
            });
        singles.drain().for_each(|food| {
            allergen_map.values_mut().for_each(|foods| {
                if foods.len() > 1 {
                    let _ = foods.remove(food);
                    if foods.len() > 1 {
                        done = false;
                    }
                }
            })
        })
    }

    let allergens: HashSet<&str> = allergen_map
        .values()
        .flat_map(|s| {
            assert_eq!(s.len(), 1);
            s.iter().copied()
        })
        .collect();

    (allergen_map, allergens)
}

// foods, allergens
type Line<'a> = (Vec<&'a str>, Vec<&'a str>);

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, foods) = terminated(separated_list1(space1, alpha1), tag(" (contains "))(input)?;
    let (input, allergens) = terminated(separated_list1(tag(", "), alpha1), nomchar(')'))(input)?;

    Ok((input, (foods, allergens)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        let result = process_part1(input);
        assert_eq!(result, "5");
    }

    #[test]
    fn part2() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        let result = process_part2(input);
        assert_eq!(result, "mxmxvkd,sqjhc,fvjkl");
    }
}
