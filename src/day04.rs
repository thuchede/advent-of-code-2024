use crate::helpers;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day04.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day04.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let crossword_grid: Vec<Vec<char>> = sample.iter()
        .map(|line| line.chars().collect::<Vec<char>>()).collect();
    let mut total = 0;
    for (pos_y, line) in crossword_grid.clone().iter().enumerate() {
        for (pos_x, &c) in line.iter().enumerate() {
            if c == 'X' {
                let count = find_word_at_pos(pos_x, pos_y, &crossword_grid);
                total += count;
            }
        }
    }
    total
}

fn find_word_at_pos(pos_x: usize, pos_y: usize, crossword_grid: &Vec<Vec<char>>) -> i64 {
    let mut total = 0;

    // →
    if pos_x+3 < crossword_grid[pos_y].len()
        && crossword_grid[pos_y][pos_x+1] == 'M'
        && crossword_grid[pos_y][pos_x+2] == 'A'
        && crossword_grid[pos_y][pos_x+3] == 'S' {
        total+=1;
    }
    // ←
    if pos_x >= 3
        && crossword_grid[pos_y][pos_x-1] == 'M'
        && crossword_grid[pos_y][pos_x-2] == 'A'
        && crossword_grid[pos_y][pos_x-3] == 'S' {
        total+=1;
    }
    // ↑
    if pos_y >= 3
        && crossword_grid[pos_y-1][pos_x] == 'M'
        && crossword_grid[pos_y-2][pos_x] == 'A'
        && crossword_grid[pos_y-3][pos_x] == 'S' {
        total+=1;
    }
    // ↓
    if pos_y + 3 < crossword_grid.len()
        && crossword_grid[pos_y+1][pos_x] == 'M'
        && crossword_grid[pos_y+2][pos_x] == 'A'
        && crossword_grid[pos_y+3][pos_x] == 'S' {
        total+=1;
    }
    // ↖
    if pos_y >= 3 && pos_x >= 3
        && crossword_grid[pos_y-1][pos_x-1] == 'M'
        && crossword_grid[pos_y-2][pos_x-2] == 'A'
        && crossword_grid[pos_y-3][pos_x-3] == 'S' {
        total+=1;
    }
    // ↗
    if pos_y >= 3 && pos_x + 3 < crossword_grid[pos_y].len()
        && crossword_grid[pos_y-1][pos_x+1] == 'M'
        && crossword_grid[pos_y-2][pos_x+2] == 'A'
        && crossword_grid[pos_y-3][pos_x+3] == 'S' {
        total+=1;
    }
    // ↙
    if pos_y + 3 < crossword_grid.len() && pos_x >= 3
        && crossword_grid[pos_y+1][pos_x-1] == 'M'
        && crossword_grid[pos_y+2][pos_x-2] == 'A'
        && crossword_grid[pos_y+3][pos_x-3] == 'S' {
        total+=1;
    }
    // ↘
    if pos_y + 3 < crossword_grid.len() && pos_x + 3 < crossword_grid[pos_y].len()
        && crossword_grid[pos_y+1][pos_x+1] == 'M'
        && crossword_grid[pos_y+2][pos_x+2] == 'A'
        && crossword_grid[pos_y+3][pos_x+3] == 'S' {
        total+=1;
    }
    total
}


fn find_x_word_at_pos(pos_x: usize, pos_y: usize, crossword_grid: &Vec<Vec<char>>) -> i64 {
     if !(pos_x+1 < crossword_grid[pos_y].len()
         && pos_x >= 1
         && pos_y+1 < crossword_grid.len()
         && pos_y >= 1) {
         return 0
     }

    // M.M
    // .A.
    // S.S
    if crossword_grid[pos_y-1][pos_x-1] == 'M'
        && crossword_grid[pos_y-1][pos_x+1] == 'M'
        && crossword_grid[pos_y+1][pos_x-1] == 'S'
        && crossword_grid[pos_y+1][pos_x+1] == 'S' {
        return 1
    }

    // M.S
    // .A.
    // M.S
    if crossword_grid[pos_y-1][pos_x-1] == 'M'
        && crossword_grid[pos_y-1][pos_x+1] == 'S'
        && crossword_grid[pos_y+1][pos_x-1] == 'M'
        && crossword_grid[pos_y+1][pos_x+1] == 'S' {
        return 1
    }

    // S.S
    // .A.
    // M.M
    if crossword_grid[pos_y-1][pos_x-1] == 'S'
        && crossword_grid[pos_y-1][pos_x+1] == 'S'
        && crossword_grid[pos_y+1][pos_x-1] == 'M'
        && crossword_grid[pos_y+1][pos_x+1] == 'M' {
        return 1
    }

    // S.M
    // .A.
    // S.M
    if crossword_grid[pos_y-1][pos_x-1] == 'S'
        && crossword_grid[pos_y-1][pos_x+1] == 'M'
        && crossword_grid[pos_y+1][pos_x-1] == 'S'
        && crossword_grid[pos_y+1][pos_x+1] == 'M' {
        return 1
    }
    0
}

fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let crossword_grid: Vec<Vec<char>> = sample.iter()
        .map(|line| line.chars().collect::<Vec<char>>()).collect();
    let mut total = 0;
    for (pos_y, line) in crossword_grid.clone().iter().enumerate() {
        for (pos_x, &c) in line.iter().enumerate() {
            if c == 'A' {
                let count = find_x_word_at_pos(pos_x, pos_y, &crossword_grid);
                total += count;
            }
        }
    }
    total
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 2551);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample04.txt");
        assert_eq!(res, 18);
    }

    #[test]
    fn test_find_word_at_pos() {
        // 9 col 0..8
        // 7 row 0..6
        // x at 4th row: y 3
        // x at 5th col: x 4
        let grid = vec![
            ".S..S..S.".chars().collect::<Vec<char>>(),
            "..A.A.A..".chars().collect::<Vec<char>>(),
            "...MMM...".chars().collect::<Vec<char>>(),
            ".SAMXMAS.".chars().collect::<Vec<char>>(),
            "...MMM...".chars().collect::<Vec<char>>(),
            "..A.A.A..".chars().collect::<Vec<char>>(),
            ".S..S..S.".chars().collect::<Vec<char>>(),
        ];
        println!(">>{}", grid[3][4]);
        let res = find_word_at_pos(4,3, &grid);
        assert_eq!(res, 8);
    }

    #[test]
    fn test_find_x_word_at_pos() {
        let grid = vec![
            "S.M".chars().collect::<Vec<char>>(),
            ".A.".chars().collect::<Vec<char>>(),
            "S.M".chars().collect::<Vec<char>>(),
        ];
        let res = find_x_word_at_pos(1,1, &grid);
        assert_eq!(res, 1);
        let grid = vec![
            "S.S".chars().collect::<Vec<char>>(),
            ".A.".chars().collect::<Vec<char>>(),
            "M.M".chars().collect::<Vec<char>>(),
        ];
        let res = find_x_word_at_pos(1,1, &grid);
        assert_eq!(res, 1);
        let grid = vec![
            "M.S".chars().collect::<Vec<char>>(),
            ".A.".chars().collect::<Vec<char>>(),
            "M.S".chars().collect::<Vec<char>>(),
        ];
        let res = find_x_word_at_pos(1,1, &grid);
        assert_eq!(res, 1);
        let grid = vec![
            "M.M".chars().collect::<Vec<char>>(),
            ".A.".chars().collect::<Vec<char>>(),
            "S.S".chars().collect::<Vec<char>>(),
        ];
        let res = find_x_word_at_pos(1,1, &grid);
        assert_eq!(res, 1);
        let grid = vec![
            "A..".chars().collect::<Vec<char>>(),
            ".M.".chars().collect::<Vec<char>>(),
            "...".chars().collect::<Vec<char>>(),
        ];
        let res = find_x_word_at_pos(0,0, &grid);
        assert_eq!(res, 0);
        let grid = vec![
            "A.S".chars().collect::<Vec<char>>(),
            ".A.".chars().collect::<Vec<char>>(),
            "M.S".chars().collect::<Vec<char>>(),
        ];
        let res = find_x_word_at_pos(1,1, &grid);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample04.txt");
        assert_eq!(res, 9);
    }

    #[test]
    fn test_read_from_v2b() {
        let res = read_from_v2("src/input/sample04b.txt");
        assert_eq!(res, 5);
    }

    #[test]
    fn test_read_from_v2c() {
        let res = read_from_v2("src/input/sample04c.txt");
        assert_eq!(res, 4);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 1985);
    }
}
