use std::cmp::PartialEq;
use nom::character::complete;
use nom::bytes::complete::tag;
use crate::helpers;
use nom::multi::separated_list0;
use nom::{IResult, Parser};
use itertools::Itertools;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day02.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day02.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let reports: Vec<Vec<i64>> = sample.iter().map(|e| parse_report(e.as_str()).unwrap().1).collect();
    let valid_reports = reports.iter().filter(|&report| is_following_rules(report)).count() as i64;
    valid_reports
}

fn parse_report(line: &str) -> IResult<&str, Vec<i64>> {
    let res = separated_list0(tag(" "), complete::i64).parse(line)?;
    Ok(res)
}

fn is_following_rules(report: &Vec<i64>) -> bool {
    let res: Vec<i64> = report.iter().tuple_windows().map(|(a, b)| b-a).collect();
    let increase = res.iter().all(|&a| a>0);
    let decrease = res.iter().all(|&a| a<0);
    let accepted_step = res.iter().all(|&a| a.abs()>=1 && a.abs()<=3);
    (increase || decrease) && accepted_step
}

fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let reports: Vec<Vec<i64>> = sample.iter().map(|e| parse_report(e.as_str()).unwrap().1).collect();
    let valid_reports = reports.iter().filter(|&report| is_following_rules_with_dampener(report)).count() as i64;
    valid_reports
}

fn is_following_rules_with_dampener(report: &Vec<i64>) -> bool {
    if is_following_rules(report) {
        return true
    }
    for i in 0..report.len() {
        let mut updated_report = report.clone();
        updated_report.remove(i);
        if is_following_rules(&updated_report) {
            return true
        }
    }
    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 670);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample02.txt");
        assert_eq!(res, 2);
    }

    #[test]
    fn test_parse_report() {
        let res = parse_report("0 1 2 3");
        assert_eq!(res.unwrap().1, vec![0,1,2,3]);
    }

    #[test]
    fn test_is_following_rules() {
        let res = is_following_rules(&vec![7, 6, 4, 2, 1]);
        assert_eq!(res, true);
        let res = is_following_rules(&vec![1, 2, 7, 8, 9]);
        assert_eq!(res, false);
        let res = is_following_rules(&vec![9, 7, 6, 2, 1]);
        assert_eq!(res, false);
        let res = is_following_rules(&vec![1, 3, 2, 4, 5]);
        assert_eq!(res, false);
        let res = is_following_rules(&vec![8, 6, 4, 4, 1]);
        assert_eq!(res, false);
        let res = is_following_rules(&vec![1, 3, 6, 7, 9]);
        assert_eq!(res, true);
    }

    #[test]
    fn test_is_following_rules_with_dampener() {
        let res = is_following_rules_with_dampener(&vec![7, 6, 4, 2, 1]);
        assert_eq!(res, true);
        let res = is_following_rules_with_dampener(&vec![1, 2, 7, 8, 9]);
        assert_eq!(res, false);
        let res = is_following_rules_with_dampener(&vec![9, 7, 6, 2, 1]);
        assert_eq!(res, false);
        let res = is_following_rules_with_dampener(&vec![1, 3, 2, 4, 5]);
        assert_eq!(res, true);
        let res = is_following_rules_with_dampener(&vec![8, 6, 4, 4, 1]);
        assert_eq!(res, true);
        let res = is_following_rules_with_dampener(&vec![1, 3, 6, 7, 9]);
        assert_eq!(res, true);
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample02.txt");
        assert_eq!(res, 4);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 700);
    }
}
