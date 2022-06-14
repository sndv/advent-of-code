use std::collections::HashMap;
use std::collections::HashSet;

pub fn day04a(input: &str) -> i64 {
    input
        .trim_end()
        .split("\n\n")
        .filter(|passport| {
            let fields: HashSet<&str> = passport
                .split_whitespace()
                .map(|field| field.splitn(2, ':').nth(0).unwrap())
                .collect();
            HashSet::from(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"])
                .difference(&fields)
                .count()
                == 0
        })
        .count() as i64
}

pub fn day04b(input: &str) -> i64 {
    input
        .trim_end()
        .split("\n\n")
        .map(|passport| {
            passport
                .split_whitespace()
                .map(|field| {
                    let field_split: Vec<&str> = field.splitn(2, ':').collect();
                    (field_split[0], field_split[1])
                })
                .collect::<HashMap<&str, &str>>()
        })
        .filter(|passport| {
            let fields = passport.keys().map(|key| *key).collect::<HashSet<&str>>();
            HashSet::from(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"])
                .difference(&fields)
                .count()
                == 0
        })
        .filter(|passport| {
            let birth_year = passport.get("byr").unwrap().parse::<i32>().unwrap();
            birth_year >= 1920 && birth_year <= 2002
        })
        .filter(|passport| {
            let issue_year = passport.get("iyr").unwrap().parse::<i32>().unwrap();
            issue_year >= 2010 && issue_year <= 2020
        })
        .filter(|passport| {
            let expiration_year = passport.get("eyr").unwrap().parse::<i32>().unwrap();
            expiration_year >= 2020 && expiration_year <= 2030
        })
        .filter(|passport| {
            let height_full = passport.get("hgt").unwrap();
            let (height_s, unit) = height_full.split_at(height_full.len() - 2);
            if unit == "cm" {
                let height = height_s.parse::<i32>().unwrap();
                height >= 150 && height <= 193
            } else if unit == "in" {
                let height = height_s.parse::<i32>().unwrap();
                height >= 59 && height <= 76
            } else {
                false
            }
        })
        .filter(|passport| {
            let hair_color = passport.get("hcl").unwrap();
            hair_color.len() == 7
                && hair_color.chars().nth(0).unwrap() == '#'
                && hair_color
                    .chars()
                    .filter(|ch| "1234567890abcdef".contains(*ch))
                    .count()
                    == 6
        })
        .filter(|passport| {
            let eye_color = passport.get("ecl").unwrap();
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(eye_color)
        })
        .filter(|passport| {
            let passport_id = passport.get("pid").unwrap();
            passport_id.len() == 9
                && passport_id
                    .chars()
                    .filter(|ch| "1234567890".contains(*ch))
                    .count()
                    == 9
        })
        .count() as i64
}
