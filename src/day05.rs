use std::cmp::Ordering;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::sequence::separated_pair;
use crate::helpers;
use nom::{IResult, Parser};
use nom::multi::separated_list0;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day05.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day05.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let (pages_ordering, pages_to_produce) = split_group(&sample);
    let ordering = pages_ordering.iter().map(|line| parse_ordering_rule(line.as_str()).unwrap().1).collect::<Vec<(i64, i64)>>();
    let production = pages_to_produce.iter().map(|line| parse_production_order(line.as_str()).unwrap().1).collect::<Vec<Vec<i64>>>();

    let res = production.iter().filter(|&prod| check_rules(prod, &ordering)).map(|l| l.get(l.len()/2).unwrap()).sum();
    res
}

fn parse_ordering_rule(line: &str) -> IResult<&str, (i64, i64)> {
    let (i, p): (&str, (i64, i64)) = separated_pair(complete::i64::<&str, ()>, tag("|"), complete::i64::<&str, ()>).parse(line).unwrap();
    Ok((i, p))
}

fn parse_production_order(line: &str) -> IResult<&str, Vec<i64>> {
    let (i, v): (&str, Vec<i64>) = separated_list0(tag(","), complete::i64::<&str, ()>).parse(line).unwrap();
    Ok((i, v))
}

fn check_rules(production_order: &Vec<i64>, rules: &Vec<(i64, i64)>) -> bool {
    rules.iter().all(|(first, second)|{
        let position_first = production_order.iter().find_position(|&e| e == first);
        let position_second = production_order.iter().find_position(|&e| e == second);
        if let Some((idx_first, _)) = position_first {
            if let Some((idx_second, _)) = position_second {
                return idx_first < idx_second
            }
        }
        true
    })
}

fn split_group(lines: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut pages_ordering = Vec::<String>::new();
    let mut pages_to_produce = Vec::<String>::new();
    let mut section = &mut pages_ordering;
    for line in lines.iter() {
        if line == "" {
            section = &mut pages_to_produce;
        } else {
            section.push(line.clone())
        }
    }
    (pages_ordering, pages_to_produce)
}

fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let (pages_ordering, pages_to_produce) = split_group(&sample);
    let ordering = pages_ordering.iter().map(|line| parse_ordering_rule(line.as_str()).unwrap().1).collect::<Vec<(i64, i64)>>();
    let production = pages_to_produce.iter().map(|line| parse_production_order(line.as_str()).unwrap().1).collect::<Vec<Vec<i64>>>();

    let mut wrong_order: Vec<Vec<i64>> = production.into_iter().filter(|prod| !check_rules(prod, &ordering)).collect();
    let ordered = wrong_order.iter_mut().map(|line| {
        line.sort_by(|b, a| {
            let rule_sorted = ordering.iter().find(|(f, s)| (a == f && b == s));
            if rule_sorted.is_some() {
                return Ordering::Greater
            }
            let rule_unsorted = ordering.iter().find(|(f, s)| (a == s && b == f));
            if rule_unsorted.is_some() {
                return Ordering::Less
            }
            Ordering::Equal
        });
        line.clone()
    }).collect::<Vec<Vec<i64>>>();
    let res = ordered.iter().map(|l| l.get(l.len()/2).unwrap()).sum();
    res
}


#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 5329);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample05.txt");
        assert_eq!(res, 143);
    }

    #[test]
    fn test_split_group() {
        let v = vec!["1".to_string(), "2".to_string(), "".to_string(), "a".to_string(),"b".to_string()];
        let res = split_group(&v);
        assert_eq!(res.0, vec!["1".to_string(), "2".to_string()]);
        assert_eq!(res.1, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn test_parse_ordering_rule() {
        let line = "55|90";
        let res = parse_ordering_rule(line).unwrap().1;
        assert_eq!(res, (55, 90));
    }

    #[test]
    fn test_parse_production_order() {
        let line = "55,90";
        let res = parse_production_order(line).unwrap().1;
        assert_eq!(res, vec![55, 90]);
    }

    #[test]
    fn test_check_rule() {
        let rule = (55, 90);
        let prod = vec![55, 90];
        let prod_2 = vec![90, 55];
        let prod_3 = vec![55, 91];

        let checked = check_rules(&prod, &vec![rule]);
        let not_checked = check_rules(&prod_2, &vec![rule]);
        let unrelated = check_rules(&prod_3, &vec![rule, (42, 74)]);

        assert!(checked);
        assert!(!not_checked);
        assert!(unrelated);
    }


    #[test]
    fn test_sort_by() {
        // trying sort_by algo
        let rules: Vec<(i64, i64)> = vec![
            (47,53),
            (97,13),
            (97,61),
            (97,47),
            (75,29),
            (61,13),
            (75,53),
            (29,13),
            (97,29),
            (53,29),
            (61,53),
            (97,53),
            (61,29),
            (47,13),
            (75,47),
            (97,75),
            (47,61),
            (75,61),
            (47,29),
            (75,13),
            (53,13),
        ];
        // let mut line: Vec<i64> = vec![75,97,47,61,53];
        let mut line: Vec<i64> = vec![61,13,29];
        line.sort_by(|b, a| {
            let rule_sorted = rules.iter().find(|(f,s)| (a==f && b==s));
            if rule_sorted.is_some() {
                return Ordering::Greater
            }
            let rule_unsorted = rules.iter().find(|(f,s)| (a==s && b==f));
            if rule_unsorted.is_some() {
                return Ordering::Less
            }
            Ordering::Equal
        });
        // assert_eq!(line, vec![97, 75, 47, 61, 53]);
        assert_eq!(line, vec![61, 29, 13]);
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample05.txt");
        assert_eq!(res, 123);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 5833);
    }
}
