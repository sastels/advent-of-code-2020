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
                .split(" ")
                .map(|s| s.to_string())
                .collect(),
            allergens: parts
                .next()
                .unwrap()
                .split(" ")
                .map(|s| s.to_string())
                .collect(),
        }
    }
}

pub fn solve_a(data: &[String]) -> usize {
    let foods: Vec<Food> = data.iter().map(|line| Food::new(line)).collect();

    let all_allergens: HashSet<String> =
        foods
            .iter()
            .map(|food| &food.allergens)
            .fold(HashSet::new(), |acc, x| {
                let union: HashSet<_> = acc.union(&x).collect();
                let union: HashSet<String> = union.iter().map(|s| s.to_string()).collect();
                union
            });
    let all_ingredients: HashSet<String> =
        foods
            .iter()
            .map(|food| &food.ingredients)
            .fold(HashSet::new(), |acc, x| {
                let union: HashSet<_> = acc.union(&x).collect();
                let union: HashSet<String> = union.iter().map(|s| s.to_string()).collect();
                union
            });

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

    let maybe_bad = possible_ingredients
        .values()
        .fold(HashSet::new(), |acc, x| {
            let union: HashSet<_> = acc.union(&x).collect();
            let union: HashSet<String> = union.iter().map(|s| s.to_string()).collect();
            union
        });

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
        .fold(0, |acc, x| acc + x)
}

pub fn solve_b(_data: &[String]) -> usize {
    unimplemented!()
}
