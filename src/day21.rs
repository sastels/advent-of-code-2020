use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Food {
    pub ingredients: HashSet<String>,
    pub allergens: HashSet<String>,
}

// trh fvjkl sbzzf mxmxvkd (contains dairy)

impl Food {
    pub fn new(line: &str) -> Self {
        let line = line.replace(")", "").replace(",", "");
        let mut parts = line.split(" (contains ");
        Food {
            ingredients: parts
                .next()
                .unwrap()
                .split(' ')
                .map(|s| s.to_string())
                .collect(),
            allergens: parts
                .next()
                .unwrap()
                .split(' ')
                .map(|s| s.to_string())
                .collect(),
        }
    }
}

pub fn find_all_allergens(foods: &[Food]) -> HashSet<String> {
    let all_allergens: HashSet<String> =
        foods
            .iter()
            .map(|food| &food.allergens)
            .fold(HashSet::new(), |acc, x| {
                let union: HashSet<_> = acc.union(&x).collect();
                let union: HashSet<String> = union.iter().map(|s| s.to_string()).collect();
                union
            });
    all_allergens
}

pub fn find_all_ingredients(foods: &[Food]) -> HashSet<String> {
    let all_ingredients: HashSet<String> =
        foods
            .iter()
            .map(|food| &food.ingredients)
            .fold(HashSet::new(), |acc, x| {
                let union: HashSet<_> = acc.union(&x).collect();
                let union: HashSet<String> = union.iter().map(|s| s.to_string()).collect();
                union
            });
    all_ingredients
}

pub fn find_possible_matches(foods: &[Food]) -> HashMap<String, HashSet<String>> {
    let all_allergens = find_all_allergens(&foods);
    let all_ingredients = find_all_ingredients(&foods);
    let mut possible_ingredients: HashMap<String, HashSet<String>> = HashMap::new();
    for allergen in all_allergens {
        let ingredients = foods
            .iter()
            .filter(|food| food.allergens.contains(&allergen))
            .map(|food| &food.ingredients)
            .fold(all_ingredients.clone(), |acc, x| {
                let intersection: HashSet<_> = acc.intersection(&x).collect();
                let intersection: HashSet<String> =
                    intersection.iter().map(|s| s.to_string()).collect();
                intersection
            });
        possible_ingredients.insert(allergen.clone(), ingredients);
    }
    possible_ingredients
}

pub fn solve_a(data: &[String]) -> usize {
    let foods: Vec<Food> = data.iter().map(|line| Food::new(line)).collect();

    let maybe_bad = find_possible_matches(&foods)
        .values()
        .fold(HashSet::new(), |acc, x| {
            let union: HashSet<_> = acc.union(&x).collect();
            let union: HashSet<String> = union.iter().map(|s| s.to_string()).collect();
            union
        });

    let all_ingredients = find_all_ingredients(&foods);

    let diff: HashSet<_> = all_ingredients.difference(&maybe_bad).collect();
    let good_ingredients: HashSet<_> = diff.iter().map(|s| s.to_string()).collect();

    foods
        .iter()
        .map(|food| {
            food.ingredients
                .intersection(&good_ingredients)
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

pub fn single_ingredient(list: &HashSet<String>) -> Option<String> {
    let mut list: Vec<String> = list.iter().map(|s| s.to_string()).collect();

    if list.len() == 1 {
        list.drain(..).next()
    } else {
        None
    }
}

pub fn solve_b(data: &[String]) -> String {
    let foods: Vec<Food> = data.iter().map(|line| Food::new(line)).collect();
    let mut matches = find_possible_matches(&foods);
    let all_allergens = find_all_allergens(&foods);

    let mut pairs: Vec<(String, String)> = vec![];
    loop {
        let mut changed = false;

        for allergen in &all_allergens {
            let matching_ingredient = single_ingredient(&matches.get(allergen).unwrap());
            if let Some(ingredient) = matching_ingredient {
                pairs.push((allergen.clone(), ingredient.clone()));
                for a in &all_allergens {
                    if matches.get_mut(a).unwrap().contains(&ingredient) {
                        changed = true;
                    }
                    matches.get_mut(a).unwrap().remove(&ingredient);
                }
            }
        }
        if !changed {
            break;
        }
    }

    pairs.sort();
    pairs.iter().map(|(_, i)| i).join(",")
}
