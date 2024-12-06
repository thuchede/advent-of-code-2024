use std::thread::current;
use itertools::Itertools;
use nom::multi::count;
use crate::helpers;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day06.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day06.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let area_map = sample.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let starting_position = find_starting_position(&area_map);
    // println!("{:?}", starting_position);
    patrol(&area_map, starting_position.unwrap())
}

fn find_starting_position(area: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    area.iter().enumerate().find_map(|(y_position, line)| {
        let maybe_x_position = line.iter()
            .find_position(|&&column|{
                column == '^'
            });
        if maybe_x_position.is_none() {
            return None
        }
        Some((maybe_x_position.unwrap().0, y_position))
    })
}

#[derive(Eq, PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn patrol<'a>(area: &Vec<Vec<char>>, start_pos: (usize, usize)) -> i64 {
    let mut cloned_area = area.clone();
    let mut patrolled_area: Vec<Vec<&mut char>> = cloned_area.iter_mut().map(|x| x.iter_mut().collect::<Vec<&mut char>>()).collect::<Vec<Vec<&mut char>>>();
    let (mut current_x_pos, mut current_y_pos):  (i64, i64) = (start_pos.0 as i64, start_pos.1 as i64);
    let mut currentDirection = Direction::North;
    // TODO: need to update first iteration?
    // let mut step =0;
    while !(
        current_y_pos < 0
        || current_y_pos >= patrolled_area.len() as i64
        || current_x_pos < 0
        || current_x_pos >= patrolled_area.get(current_y_pos as usize).unwrap().len() as i64
    ) {

        // println!("PATROLL...ing in {:?} to ({current_x_pos},{current_y_pos})\n", currentDirection);
        // println!("  [ 0 ,  1 ,  2 ,  3 ,  4 ,  5 ,  6 ,  7 ,  8 ,  9 ]");
        // for (p, x) in patrolled_area.iter().enumerate() {
        //     println!("{p} {:?}", x);
        // }
        let mut line = patrolled_area.get_mut(current_y_pos as usize).unwrap();
        **(line.get_mut(current_x_pos as usize).unwrap()) = 'X';

        if currentDirection == Direction::North {
            if current_y_pos - 1 >= 0 && **(patrolled_area.get_mut((current_y_pos-1) as usize).unwrap().get_mut((current_x_pos) as usize)).unwrap() == '#' {
                // println!("  [ 0 ,  1 ,  2 ,  3 ,  4 ,  5 ,  6 ,  7 ,  8 ,  9 ]");
                currentDirection = Direction::East;
                continue;
            } else {
                current_y_pos -= 1;
            }

        }

        if currentDirection == Direction::East {
            if current_x_pos + 1 < patrolled_area.len() as i64 && **(patrolled_area.get_mut(current_y_pos as usize).unwrap().get_mut((current_x_pos+1) as usize)).unwrap() == '#' {
                currentDirection = Direction::South;
                continue;
            } else {
               current_x_pos += 1;
            }
        }

        if currentDirection == Direction::South {
            if current_y_pos + 1 < patrolled_area.len() as i64 && **(patrolled_area.get_mut((current_y_pos+1) as usize).unwrap().get_mut((current_x_pos) as usize)).unwrap() == '#' {
                currentDirection = Direction::West;
                continue;
            } else {
                current_y_pos += 1;
            }
        }

        if currentDirection == Direction::West {
            if current_x_pos - 1 >= 0 && **(patrolled_area.get_mut(current_y_pos as usize).unwrap().get_mut((current_x_pos-1) as usize)).unwrap() == '#' {
                currentDirection = Direction::North;
                continue;
            } else {
                current_x_pos -= 1;
            }
        }

        // current_x_pos -=1;
        // println!("going to {:?}; {:?}", current_x_pos, current_y_pos);
        // step +=1;
        // if step > 50 {
        //     break;
        // }
    }

    // println!("PATROLLED\n");
    // for x in &patrolled_area {
    //     println!("{:?}", x);
    // }

    let res: i64 = patrolled_area.iter().map(|l| l.into_iter().filter(|&&&mut c| c == 'X').count() as i64).sum();
    res
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
        assert_eq!(res, 4696);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample06.txt");
        assert_eq!(res, 41);
    }

    #[test]
    fn test_find_starting_position() {
        let sample = helpers::read("src/input/sample06.txt").unwrap();
        let area_map = sample.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let res = find_starting_position(&area_map);
        assert_eq!(res, Some((4,6)));
    }

    #[test]
    fn test_patrol() {
        let sample = helpers::read("src/input/sample06.txt").unwrap();
        let area_map = sample.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let pos = find_starting_position(&area_map);
        let res = patrol(&area_map, pos.unwrap());
        assert_eq!(res, 41);
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample06.txt");
        assert_eq!(res, 6);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 0);
    }
}
