use std::collections::HashMap;
use std::collections::HashSet;

pub fn day07a(input: &str) -> i64 {
    let rules: Vec<(&str, HashSet<&str>)> = input
        .trim_end()
        .split("\n")
        .map(|rule_text| {
            let (container_bag, contents) = rule_text
                .trim_end_matches('.')
                .split_once(" bags contain ")
                .unwrap();
            let contents_list: HashSet<&str> = HashSet::from_iter(
                contents
                    .split(", ")
                    .map(|item| item.split_once(" ").unwrap().1.rsplit_once(" ").unwrap().0),
            );
            (container_bag, contents_list)
        })
        .collect();
    let mut result_bags: Vec<&str> = Vec::new();
    let mut result_bags_prev_len = 999;
    let mut curr_check = HashSet::from(["shiny gold"]);
    while result_bags.len() != result_bags_prev_len {
        result_bags_prev_len = result_bags.len();
        let new_bags: Vec<&str> = rules
            .iter()
            .filter(|(_, contents)| contents.intersection(&curr_check).count() > 0)
            .map(|(container, _)| *container)
            .collect();
        result_bags.append(&mut new_bags.clone());
        curr_check = HashSet::from_iter(new_bags.iter().map(|item| *item));
    }
    let result_bags_set: HashSet<&str> = HashSet::from_iter(result_bags.iter().map(|el| *el));
    result_bags_set.len() as i64
}

pub fn num_bags_inside(bag_type: &str, rules: &HashMap<&str, Vec<(usize, &str)>>) -> usize {
    let mut total_bags = 0;
    for (count, in_bag_type) in rules.get(bag_type).unwrap() {
        total_bags += count * (num_bags_inside(in_bag_type, rules) + 1);
    }
    total_bags
}

pub fn day07b(input: &str) -> i64 {
    let rules: HashMap<&str, Vec<(usize, &str)>> = input
        .trim_end()
        .split("\n")
        .map(|rule_text| {
            let (container_bag, contents) = rule_text
                .trim_end_matches('.')
                .split_once(" bags contain ")
                .unwrap();
            let contents_list: Vec<(usize, &str)> = if contents == "no other bags" {
                Vec::new()
            } else {
                contents
                    .split(", ")
                    .map(|item| {
                        let (count, bag_type_text) = item.split_once(" ").unwrap();
                        (
                            count.parse::<usize>().unwrap(),
                            bag_type_text.rsplit_once(" ").unwrap().0,
                        )
                    })
                    .collect()
            };
            (container_bag, contents_list)
        })
        .collect();
    num_bags_inside("shiny gold", &rules) as i64
}
