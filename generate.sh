#!/bin/sh

#define parameters which are passed in.
DAY=$1
DAY_EXACT=$(expr $DAY + 0)
source .env

echo day is $DAY_EXACT
echo session is $SESSION
#define the template.
cat  << EOF >> ./src/day$DAY.rs
use crate::helpers;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day$DAY.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day$DAY.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    0
}


fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 0);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample$DAY.txt");
        assert_eq!(res, 0);
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample$DAY.txt");
        assert_eq!(res, 0);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 0);
    }
}
EOF

touch  ./src/input/sample$DAY.txt

curl https://adventofcode.com/2024/day/$DAY_EXACT/input -H "cookie: session=$SESSION">> ./src/input/day$DAY.txt