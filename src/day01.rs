use nom::bytes::complete::tag;
use nom::character::complete;
use nom::{IResult, Parser};
use nom::sequence::{pair, preceded};
use crate::helpers;
use itertools::Itertools;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day01.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day01.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let (mut list1, mut list2): (Vec<i64>, Vec<i64>) = sample.iter()
        .map(|line| parse_row(line.as_str()).unwrap().1)
        .fold((vec![], vec![]), |mut acc, (first, second)| {
            acc.0.push(first);
            acc.1.push(second);
            acc
        });
    list1.sort();
    list2.sort();

    let diff: i64 = list1.iter().zip(list2.iter()).map(|(&a, &b)| (a-b).abs()).sum();

    diff
}

fn parse_row(line: &str) -> IResult<&str, (i64, i64)> {
    let (_, pair) = pair(complete::i64, preceded(tag("   "), complete::i64)).parse(line)?;
    Ok((line, pair))
}

fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let (mut list1, mut list2): (Vec<i64>, Vec<i64>) = sample.iter()
        .map(|line| parse_row(line.as_str()).unwrap().1)
        .fold((vec![], vec![]), |mut acc, (first, second)| {
            acc.0.push(first);
            acc.1.push(second);
            acc
        });
    list1.sort();
    // let similarity_map = list2.iter().fold(HashMap::new(), |(acc, item)| acc.get);
    let similarity_map = list2.iter().counts();

    let similarity  = list1.iter().map(|e| (*similarity_map.get(e).unwrap_or(&0usize) as i64) * e).sum::<i64>();
    println!("{:?}", similarity);
    similarity
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 1189304);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample01.txt");
        assert_eq!(res, 11);
    }

    #[test]
    fn test_parse_row() {
        let res = parse_row("13   14").unwrap().1;
        assert_eq!(res, (13, 14));
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample01.txt");
        assert_eq!(res, 31);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 24349736);
    }
}