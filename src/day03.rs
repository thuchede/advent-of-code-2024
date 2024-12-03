use nom::bytes::complete::tag;
use nom::character::complete;
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::character::complete::{anychar};
use nom::combinator::{eof, value};
use nom::multi::{many0, many_till};
use nom::sequence::{pair, preceded, separated_pair, terminated};
use crate::helpers;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day03.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day03.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let res = sample.iter().map(|line| parse_instruction(line).iter().sum::<i64>()).sum::<i64>();
    res
}

fn parse_mul(input: &str) -> IResult<&str, i64> {
    let (i, (a, b)): (&str, (i64, i64)) = terminated(
        preceded(
            tag("mul("),
            separated_pair(
                complete::i64,
                tag(","),
                complete::i64
            )
        ),
        tag(")")
    ).parse(input)?;
    Ok((i, a*b))
}

fn parse_instruction(input: &str) -> Vec<i64> {
    let (_, v): (&str, Vec<i64>) =
                many0(
                    alt((
                        parse_mul,
                        value(0, anychar)
                    )),
                ).parse(input).unwrap();
    v
}

fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    // just join all lines into one
    // a line ending with don't() should prevent mul() from next line
    let res = vec![sample.join("")].iter()
        .map(|line| parse_do_pair_complete(line).unwrap().1)
        .sum::<i64>();
    res
}

fn parse_dont_pair(input: &str) -> IResult<&str, i64> {
    let (i, (a, _)): (&str, (Vec<i64>, _)) = many_till(
        alt((
            parse_mul,
            value(0, anychar)
        )),
        alt((tag("don't()"), eof))
    ).parse(input)?;
    let v = a.iter().sum::<i64>();
    Ok((i, v))
}

fn parse_do_pair(input: &str) -> IResult<&str, i64> {
    let (i, (_, _)): (&str, (Vec<i64>, _)) = many_till(
        alt((
            parse_mul,
            value(0, anychar)
        )),
        alt((tag("do()"), eof))
    ).parse(input)?;
    Ok((i, 0))
}

fn parse_do_pair_complete(input: &str) -> IResult<&str, i64> {
    let (i, (v,_)): (&str, (Vec<(i64, i64)>, &str)) = many_till(
        pair(
            parse_dont_pair,
            parse_do_pair
        ),
        eof
    ).parse(input)?;
    let res = v.into_iter().map(|(a,b)| a+b).sum::<i64>();
    Ok((i, res))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 173517243);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample03.txt");
        assert_eq!(res, 161);
    }

    #[test]
    fn test_parse_mul() {
        let res = parse_mul("mul(5,6)").unwrap();
        assert_eq!(res.1, 30);
        let res = parse_mul("mul(5,6]");
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_instruction() {
        let res = parse_instruction("ewewmul(5,6)");
        assert_eq!(res, vec![0,0,0,0,30]);
    }

    #[test]
    fn test_parse_do_pair_complete() {
        let res = parse_do_pair_complete("mul(5,6)x").unwrap();
        assert_eq!(res.1, 30);
        let res = parse_do_pair_complete("mul(5,6)xmul(5,3)").unwrap();
        assert_eq!(res.1, 45);
        let res = parse_do_pair_complete("mul(5,6)xdon't()mul(5,3)").unwrap();
        assert_eq!(res.1, 30);
        let res = parse_do_pair_complete("mul(5,6)xdon't()mul(5,3)xmul(5,3)do()ymul(2,7)yyy").unwrap();
        assert_eq!(res.1, 44);
        let res = parse_do_pair_complete("mul(5,6)xdon't()mul(5,3)xdon't()mul(5,3)xmul(5,3)do()ymul(2,7)yyydo()ymul(2,7)yyydo()ymul(2,7)yyydon't()ymul(2,7)yyy").unwrap();
        assert_eq!(res.1, 72);
        let res = parse_do_pair_complete("don't()mul(5,3)xdon't()mul(5,3)xmul(5,3)do()ymul(2,7)yyydo()ymul(2,7)yyydo()ymul(2,7)yyydon't()ymul(2,7)yyy").unwrap();
        assert_eq!(res.1, 42);
        let res = parse_do_pair_complete("mul(1,1)xdo()mul(2,2)").unwrap();
        assert_eq!(res.1, 5);
        let res = parse_do_pair_complete("mul(1,1)xdo()mul(2,2)don't()mul(1,2)").unwrap();
        assert_eq!(res.1, 5);
        let res = parse_do_pair_complete("mul(1,1)mul(5,3)xdon't()mul(5,3)xmul(5,3)do()ymul(2,7)yyydo()ymul(2,7)yyydo()ymul(2,7)yyydon't()ymul(2,7)yyy").unwrap();
        //                                                      1      15              0        0            14              14              14                 0
        assert_eq!(res.1, 58);
        let res = parse_do_pair_complete("do()mul(5,3)xdon't()mul(5,3)xmul(5,3)do()ymul(2,7)yyydo()ymul(2,7)yyydo()ymul(2,7)yyydon't()ymul(2,7)yyy").unwrap();
        //                                                         15              0        0            14              14              14                 0
        assert_eq!(res.1, 57);
        let res = parse_do_pair_complete("do()do()mul(5,3)xdon't()do()mul(5,3)xmul(5,3)do()do()don't()ymul(2,7)yyydo()ymul(2,7)yyydo()ymul(2,7)yyydon't()ymul(2,7)yyy").unwrap();
        //                                                             15                  15       15                      0               14              14                 0
        assert_eq!(res.1, 73);
    }

    #[test]
    fn test_parse_dont() {
        let res = parse_dont_pair("don't()x").unwrap();
        assert_eq!(res.1, 0);
        let res = parse_dont_pair("don't()xmul(5,6)").unwrap();
        assert_eq!(res.1, 0);
        let res = parse_dont_pair("don't()xmul(5,6)do()").unwrap();
        assert_eq!(res.1, 0);
        let res = parse_dont_pair("don't()xmul(5,6)do()xxx").unwrap();
        assert_eq!(res.1, 0);
        let res = parse_dont_pair("mul(2,3)don't()xmul(5,6)do()xxx").unwrap();
        assert_eq!(res.1, 6);
        let res = parse_dont_pair("mul(2,3)mul(2,3)don't()xmul(5,6)do()xxx").unwrap();
        assert_eq!(res.1, 12);
        assert_eq!(res.0, "xmul(5,6)do()xxx");
        let res = parse_dont_pair("mul(5,6)x").unwrap();
        assert_eq!(res.1, 30);
        let res = parse_dont_pair("mul(5,6)don't()x").unwrap();
        assert_eq!(res.1, 30);
        let res = parse_dont_pair("x").unwrap();
        assert_eq!(res.1, 0);
        let res = parse_dont_pair("mul(1,1)xdo()mul(2,2)").unwrap();
        assert_eq!(res.1, 5);
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample03b.txt");
        assert_eq!(res, 48);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 100450138);
    }
}
