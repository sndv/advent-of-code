use std::collections::HashMap;

fn parse_input(
    input: &str,
) -> (
    Vec<(&str, Vec<(usize, usize)>)>,
    Vec<usize>,
    Vec<Vec<usize>>,
) {
    let sections: Vec<&str> = input.trim_end().split("\n\n").collect();
    let (rules, your_ticket, other_tickets) = (sections[0], sections[1], sections[2]);

    let rules_parsed: Vec<(&str, Vec<(usize, usize)>)> = rules
        .trim_end()
        .split("\n")
        .map(|line| {
            let (name, values) = line.split_once(": ").unwrap();
            let ranges: Vec<(usize, usize)> = values
                .split(" or ")
                .map(|rule_range| {
                    let split = rule_range.split_once("-").unwrap();
                    (
                        split.0.parse::<usize>().unwrap(),
                        split.1.parse::<usize>().unwrap(),
                    )
                })
                .collect();
            (name, ranges)
        })
        .collect();

    let ticket_parsed: Vec<usize> = your_ticket
        .split("\n")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|t| t.parse::<usize>().unwrap())
        .collect();

    let other_parsed: Vec<Vec<usize>> = other_tickets
        .split("\n")
        .skip(1)
        .map(|row| {
            row.split(",")
                .map(|t| t.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    (rules_parsed, ticket_parsed, other_parsed)
}

pub fn day16a(input: &str) -> i64 {
    let (rules, ticket, others) = parse_input(input);
    let unified_rules: Vec<(usize, usize)> = rules.into_iter().map(|r| r.1).flatten().collect();
    let unified_others: Vec<usize> = others.into_iter().flatten().collect();
    unified_others
        .into_iter()
        .filter(|n| unified_rules.iter().all(|(min, max)| n < min || n > max))
        .sum::<usize>() as i64
}

pub fn day16b(input: &str, field_prefix: &str) -> i64 {
    let (rules, ticket, others) = parse_input(input);
    let unified_rules: Vec<(usize, usize)> =
        rules.clone().into_iter().map(|r| r.1).flatten().collect();
    let valid_others: Vec<Vec<usize>> = others
        .into_iter()
        .filter(|ticket| {
            ticket.iter().all(|ticket_field| {
                unified_rules
                    .iter()
                    .any(|(min, max)| ticket_field >= min && ticket_field <= max)
            })
        })
        .collect();

    let valid_others_by_column =
        valid_others
            .iter()
            .fold(vec![Vec::new(); valid_others[0].len()], |acc, el| {
                acc.into_iter()
                    .enumerate()
                    .map(|(i, mut sv)| {
                        sv.push(el[i]);
                        sv
                    })
                    .collect()
            });

    let check_results: Vec<(&str, Vec<bool>)> = rules
        .iter()
        .map(|(name, ranges)| {
            let options = valid_others_by_column
                .iter()
                .map(|col| {
                    col.iter()
                        .all(|value| ranges.iter().any(|(min, max)| value >= min && value <= max))
                })
                .collect();
            (*name, options)
        })
        .collect();

    let mut remaining_fields: Vec<usize> = (0..check_results.len()).collect();
    let mut remaining_fields_sub: Vec<usize> = (0..check_results.len()).collect();
    let mut field_map: HashMap<usize, usize> = HashMap::new();

    while remaining_fields.len() > 0 {
        for (ii, field_idx) in remaining_fields.iter().enumerate() {
            let to_fix_list: Vec<_> = check_results[*field_idx]
                .1
                .iter()
                .enumerate()
                .filter(|(i, el)| remaining_fields_sub.contains(i) && **el)
                .map(|(i, _)| i)
                .collect();
            if to_fix_list.len() == 1 {
                field_map.insert(to_fix_list[0], *field_idx);
                remaining_fields.remove(ii);
                let index = remaining_fields_sub
                    .iter()
                    .position(|x| *x == to_fix_list[0])
                    .unwrap();
                remaining_fields_sub.remove(index);
                break;
            }
        }
    }

    ticket
        .iter()
        .enumerate()
        .filter(|(i, _)| {
            rules[*(field_map.get(i).unwrap())]
                .0
                .starts_with(field_prefix)
        })
        .map(|(_, v)| *v)
        .product::<usize>() as i64
}
