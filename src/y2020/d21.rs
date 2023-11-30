//! Day 21: Allergen Assessment

use std::collections::HashMap;

/// occurrence of ingredients without allergens
pub fn a(input: &Vec<&str>) -> String {
    let foods = parse_input(input);
    let ingredient_details = find_allergens(&foods);
    ingredient_details
        .iter()
        .filter_map(|(_, occurrence, allergen)| allergen.is_empty().then_some(*occurrence as usize))
        .sum::<usize>()
        .to_string()
}

/// sorted ingredients with allergens
pub fn b(input: &Vec<&str>) -> String {
    let foods = parse_input(input);
    let ingredient_details = find_allergens(&foods);
    ingredient_details
        .iter()
        .filter_map(|(ingredient, _, allergen)| (!allergen.is_empty()).then_some(*ingredient))
        .collect::<Vec<_>>()
        .join(",")
}

const NO_ALLERGEN: &str = "";
type FoodList<'a> = Vec<(Vec<&'a str>, Vec<&'a str>)>;
type IngredientDetails<'a> = Vec<(&'a str, u8, &'a str)>;

fn parse_input<'a>(input: &'a Vec<&'a str>) -> FoodList<'a> {
    input
        .iter()
        .map(|s| {
            s.split_once(" (contains ")
                .map(|(ingredients, allergens)| {
                    (
                        ingredients.split(' ').collect(),
                        allergens.strip_suffix(')').unwrap().split(", ").collect(),
                    )
                })
                .unwrap()
        })
        .collect()
}

fn find_allergens<'a>(foods: &'a FoodList<'a>) -> IngredientDetails<'a> {
    // determine which allergens each ingredient might contain
    let mut might_contain = HashMap::<&str, (u8, HashMap<&str, u8>)>::new();
    for (ingredients, allergens) in foods {
        for ingredient in ingredients {
            let (occurrence, allergen_count) = might_contain.entry(ingredient).or_default();
            *occurrence += 1;
            for allergen in allergens {
                allergen_count
                    .entry(allergen)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }

    // find highest potential occurence of each allergen per ingredient
    let mut max_allergen_count = HashMap::<&str, u8>::new();
    for (_, (_, allergen_count)) in &might_contain {
        // println!("{} ({}) - {:?}", ingredient, occurrence, allergen_count);
        for (allergen, count) in allergen_count {
            max_allergen_count
                .entry(allergen)
                .and_modify(|e| *e = *count.max(e))
                .or_insert(*count);
        }
    }
    // println!("\nmax - {:?}\n", max_allergen_count);

    // remove obviously excluded allergens from ingredients
    for (allergen, count) in &max_allergen_count {
        for (_, (_, allergen_count)) in might_contain.iter_mut() {
            if let Some(this_count) = allergen_count.get(allergen) {
                if this_count < count {
                    allergen_count.remove(allergen);
                }
            }
        }
    }
    // for (ingredient, (occurrence, allergen_count)) in &might_contain {
    //     println!("{} ({}) - {:?}", ingredient, occurrence, allergen_count);
    // }

    // remove further allergens based on uniqueness
    let mut ingredient_details = IngredientDetails::new();
    let ingredient_count = might_contain.len();
    let mut allergens_to_remove = Vec::<&str>::new();
    while ingredient_details.len() < ingredient_count {
        might_contain.retain(|&ingredient, (occurrence, allergen_count)| {
            allergen_count.retain(|k, _| !allergens_to_remove.contains(k));
            if allergen_count.len() == 1 {
                let allergen = *allergen_count.keys().next().unwrap();
                ingredient_details.push((ingredient, *occurrence, allergen));
                allergens_to_remove.push(allergen);
                false
            } else if allergen_count.is_empty() {
                ingredient_details.push((ingredient, *occurrence, NO_ALLERGEN));
                false
            } else {
                true
            }
        })
    }
    assert!(might_contain.is_empty());

    ingredient_details.sort_by_key(|(_, _, allergen)| *allergen);
    ingredient_details
}

#[test]
pub fn test() {
    let input = vec![
        "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)",
        "trh fvjkl sbzzf mxmxvkd (contains dairy)",
        "sqjhc fvjkl (contains soy)",
        "sqjhc mxmxvkd sbzzf (contains fish)",
    ];

    let foods = parse_input(&input);
    assert_eq!(foods.len(), 4);
    assert_eq!(foods[0].0, vec!["mxmxvkd", "kfcds", "sqjhc", "nhms"]);
    assert_eq!(foods[0].1, vec!["dairy", "fish"]);

    assert_eq!(a(&input), "5");
    assert_eq!(b(&input), "mxmxvkd,sqjhc,fvjkl");
}
