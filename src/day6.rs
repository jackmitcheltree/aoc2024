use std::collections::HashSet;
use std::cmp::max;

pub fn one(input: String) -> i32 {
    let grid : Vec<Vec<char>> = input.split("\n").map(|x| x.chars().collect::<Vec<_>>()).collect();
    let maxColIndex = grid[1].len() - 1;
    let maxRowIndex = grid[0].len() - 1;
    // println!("{:?} {:?}", maxRowIndex, maxColIndex);
    let dirMap : Vec<(i32, i32)> = vec![(-1,0), (0,1), (1,0), (0,-1)];
    let mut dirIter = dirMap.iter().cycle();
    let mut positions : HashSet<Vec<usize>> = Vec::new().into_iter().collect();
    let mut pos = vec![0,0];

    'findstart : for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                pos = vec![i, j];
                positions.insert(pos.clone());
                break 'findstart;
            };
        };
    };

    'guardwalks : loop {
        // println!("{:?}", pos);
        let d = dirIter.next().unwrap();
        for i in 0..max(grid.len(), grid[0].len()) {
            let rowIndex = ((pos[0] as i32) + d.0 * (i as i32)) as usize;
            let colIndex = ((pos[1] as i32) + d.1 * (i as i32)) as usize;

            // print!("{:?} {:?}", rowIndex, colIndex);

            if rowIndex <= maxRowIndex && colIndex <= maxColIndex {
                // println!(" {:?}", grid[rowIndex][colIndex]);

                if grid[rowIndex][colIndex] == '.' || grid[rowIndex][colIndex] == '^' {
                    positions.insert(vec![rowIndex, colIndex]);

                } else if grid[rowIndex][colIndex] == '#' {
                    pos = vec![((pos[0] as i32) + d.0 * ((i-1) as i32)) as usize, ((pos[1] as i32) + d.1 * ((i-1) as i32)) as usize];
                    // println!("{:?}", pos);
                    continue 'guardwalks;
                };
            } else {
                break 'guardwalks;
            };
        };
    };
    // println!("{:?}", positions);
    positions.len() as i32

}

pub fn two(input: String) -> i32 {
    0
}


#[cfg(test)]
mod sample_case {
    use super::*;

    #[test]
    fn sample_one() {
        let input : String = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...".to_string();
        let check = 41i32;

        assert_eq!(one(input), check)
    }

    #[test]
    fn sample_two() {
        let input : String = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...".to_string();
        let check = 0i32;

        assert_eq!(two(input), check)
    }
}