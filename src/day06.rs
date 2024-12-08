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

// hitting road block clockwise
enum LoopCase {
    StartsAtTwelve = 0, // 12-3-6-9
    StartsAtThree = 1, // 3-6-9-12
    StartsAtSix = 2, // 6-9-12-3
    StartsAtNine = 3, // 9-12-3-6
}

fn patrol_count_loop<'a>(area: &Vec<Vec<char>>, start_pos: (usize, usize)) -> i64 {
    let mut cloned_area = area.clone();
    let mut patrolled_area: Vec<Vec<&mut char>> = cloned_area.iter_mut().map(|x| x.iter_mut().collect::<Vec<&mut char>>()).collect::<Vec<Vec<&mut char>>>();
    let (mut current_x_pos, mut current_y_pos):  (i64, i64) = (start_pos.0 as i64, start_pos.1 as i64);
    let mut currentDirection = Direction::North;
    // TODO: need to update first iteration?
    let mut loop_count = 0;
    let last_blocks: (Option<(usize, usize)>,Option<(usize, usize)>,Option<(usize, usize)>) = (None, None, None);
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

    // let res: i64 = patrolled_area.iter().map(|l| l.into_iter().filter(|&&&mut c| c == 'X').count() as i64).sum();
    loop_count
}

fn try_loop(area: &Vec<Vec<char>>, last_blocks: (Option<(usize, usize)>,Option<(usize, usize)>,Option<(usize, usize)>), position: (usize, usize)) -> bool {
    let (first_block, _, third_block) = last_blocks;
    if first_block.is_none() {
        return false
    }
    false
}

// case 1 StartsAtTwelve
//  .a..
//  ...b
//  d... ==> d.x = a.x-1; d.y = c.y-1
//  ..c.

// case 2 StartsAtThree
//  .d..
//  ...a
//  c... ==> d.x = c.x+1; d.y = a.y-1
//  ..b.

// case 3 StartsAtSix
//  .c..
//  ...d
//  b... ==> d.x = a.x+1; d.y = c.y+1
//  ..a.

// case 4 StartsAtNine
//  .b..
//  ...c
//  a... ==> d.x = c.x-1; d.y = a.y+1
//  ..d.
fn get_potential_block_position(last_blocks: (Option<(usize, usize)>,Option<(usize, usize)>,Option<(usize, usize)>), loop_case: LoopCase, max_x: usize, max_y:usize) -> Option<(usize, usize)> {
    let (first_block, _, third_block) = last_blocks;
    if first_block.is_none() {
        return None
    }
    if let Some((first_x, first_y)) = first_block {
        if let Some((third_x, third_y)) = third_block {
            return match loop_case{
                LoopCase::StartsAtTwelve => if first_x > 0 && third_y > 0 { Some((first_x - 1, third_y - 1))} else { None }
                LoopCase::StartsAtThree => if third_x < max_x && first_y > 0 { Some((third_x + 1, first_y - 1))} else { None }
                LoopCase::StartsAtSix => if first_x < max_x && third_y < max_y { Some((first_x + 1, third_y + 1))} else { None }
                LoopCase::StartsAtNine => if third_x > 0 && first_y < max_y { Some((third_x - 1, first_y + 1))} else { None }
            }
        }
    }
    None
}

// look for a direct path to next potential road block?
// __OR__ travel to next block before calling this one and look for a direct path to first road block?
// __OR__ check both to and from paths ??
//
//   0123    StartsAtTwelve from 2,3 to 0,2
// 0 .#..
// 1 .###
// 2 #^..
// 3 ..#.
//
//   0123    StartsAtThree from 0,2 to 1,0
// 0 .#..
// 1 ..##
// 2 #^#.
// 3 ..#.
//
//   0123    StartsAtSix from 1,0 to 3,1
// 0 .#..
// 1 ...#
// 2 #^#.
// 3 ..#.
//
//   0123    StartsAtTwelve from 3,1 to 2,3
// 0 .#..
// 1 .#.#
// 2 #^..
// 3 ..#.
fn check_direct_path(area: &Vec<Vec<char>>, from: (usize, usize), to: (usize, usize), loop_case: LoopCase) -> bool {
    let (from_x, from_y) = from;
    let (to_x, to_y) = to;
    // assume to and from position are correct and within bound
    match loop_case{
        LoopCase::StartsAtTwelve =>  ((to_x+1)..=from_x).rev().all(|new_x| *area.get(to_y).unwrap().get(new_x).unwrap() != '#'),
        LoopCase::StartsAtThree => (to_y+1..=from_y).rev().all(|new_y| *area.get(new_y).unwrap().get(to_x).unwrap() != '#'),
        LoopCase::StartsAtSix => (from_x..to_x).all(|new_x| *area.get(to_y).unwrap().get(new_x).unwrap() != '#'),
        LoopCase::StartsAtNine => (from_y..to_y).all(|new_y| *area.get(new_y).unwrap().get(to_x).unwrap() != '#'),
    }
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
    fn test_get_potential_block_position() {
        let previous_blocks = (None, Some((0,1)), Some((3,1)));
        let res = get_potential_block_position(previous_blocks, LoopCase::StartsAtTwelve, 4, 4);
        assert_eq!(res, None);
        let previous_blocks = (Some((1,0)), Some((3,1)), Some((2,3)));
        let res = get_potential_block_position(previous_blocks, LoopCase::StartsAtTwelve, 4, 4);
        assert_eq!(res, Some((0,2)));
        let previous_blocks = (Some((3,1)), Some((2,3)), Some((0,2)));
        let res = get_potential_block_position(previous_blocks, LoopCase::StartsAtThree, 4, 4);
        assert_eq!(res, Some((1,0)));
        let previous_blocks = (Some((2,3)), Some((0,2)), Some((1,0)));
        let res = get_potential_block_position(previous_blocks, LoopCase::StartsAtSix, 4, 4);
        assert_eq!(res, Some((3,1)));
        let previous_blocks = (Some((0,2)), Some((1,0)), Some((3,1)));
        let res = get_potential_block_position(previous_blocks, LoopCase::StartsAtNine, 4, 4);
        assert_eq!(res,  Some((2,3)));
    }

    #[test]
    fn test_check_direct_path() {
        let area = vec![
            vec!['.', '#', '.', '.'],
            vec!['.', '.', '.', '#'],
            vec!['#', '^', '.', '.'],
            vec!['.', '.', '#', '.'],
        ];
        let res = check_direct_path(&area, (2,3), (0,2),  LoopCase::StartsAtTwelve);
        assert_eq!(res, true);
        let area = vec![
            vec!['.', '#', '.', '.'],
            vec!['.', '.', '.', '#'],
            vec!['#', '^', '.', '.'],
            vec!['.', '.', '#', '.'],
        ];
        let res = check_direct_path(&area, (0,2), (1,0), LoopCase::StartsAtThree);
        assert_eq!(res, true);
        let area = vec![
            vec!['.', '#', '.', '.'],
            vec!['.', '.', '.', '#'],
            vec!['#', '^', '.', '.'],
            vec!['.', '.', '#', '.'],
        ];
        let res = check_direct_path(&area, (1,0), (3,1), LoopCase::StartsAtSix);
        assert_eq!(res, true);
        let area = vec![
            vec!['.', '#', '.', '.'],
            vec!['.', '.', '.', '#'],
            vec!['#', '^', '.', '.'],
            vec!['.', '.', '#', '.'],
        ];
        let res = check_direct_path(&area, (3,1), (2,3), LoopCase::StartsAtNine);
        assert_eq!(res, true);
        let area = vec![
            vec!['.', '#', '.', '.'],
            vec!['.', '#', '.', '#'],
            vec!['#', '^', '#', '.'],
            vec!['.', '.', '#', '.'],
        ];
        let res = check_direct_path(&area, (2,3), (0,2),  LoopCase::StartsAtTwelve);
        assert_eq!(res, false);
        let res = check_direct_path(&area, (0,2), (1,0), LoopCase::StartsAtThree);
        assert_eq!(res, false);
        let res = check_direct_path(&area, (1,0), (3,1), LoopCase::StartsAtSix);
        assert_eq!(res, false);
        let res = check_direct_path(&area, (3,1), (2,3), LoopCase::StartsAtNine);
        assert_eq!(res, false);
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
