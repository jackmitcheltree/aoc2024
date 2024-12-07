use std::collections::HashSet;

pub fn one(input: String) -> i32 {
    let data : Vec<Vec<char>> = input.split("\n").map(|x| x.chars().collect::<Vec<_>>()).collect();
    let max_col_index = data[0].len() - 1;
    let max_row_index = data.len() - 1;
    let dirMap : Vec<[i32; 2]> = vec![[-1,-1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]];
    let mut count = 0i32;

    println!("{:?} {:?}", 0..data.len(), 0..data[data.len() - 1].len());

    for i in 0..data.len() {
        // println!("{:?}",data[i].len());
        // println!("{:?}",data[i]);
        for j in 0..data[data.len() - 1].len() {
            if data[i][j] == 'X' {
                for d in &dirMap {
                    if 0 <= i as i32 + 3 * d[0] && i as i32 + 3 * d[0] <= max_row_index as i32 && 0 <= j as i32 + 3 * d[1] && j as i32 + 3 * d[1] <= max_col_index as i32 {

                        if data[(i as i32 + 1 * d[0]) as usize][(j as i32 + 1 * d[1]) as usize] == 'M' &&
                        data[(i as i32 + 2 * d[0]) as usize][(j as i32 + 2 * d[1]) as usize] == 'A' &&
                        data[(i as i32 + 3 * d[0]) as usize][(j as i32 + 3 * d[1]) as usize] == 'S' {

                            count += 1;
                        };
                    };
                };

            };
        };
    };
    count
}

pub fn two(input: String) -> i32 {
    let data : Vec<Vec<char>> = input.split("\n").map(|x| x.chars().collect::<Vec<_>>()).collect();
    let max_col_index = data[0].len() - 1;
    let max_row_index = data.len() - 1;
    let checkMap : HashSet<char> = vec!['M', 'S'].into_iter().collect();
    let mut count = 0i32;

    println!("{:?} {:?}", 0..data.len(), 0..data[data.len() - 1].len());

    for i in 1..(data.len() - 1) {
        // println!("{:?}",data[i].len());
        // println!("{:?}",data[i]);
        for j in 1..(data[data.len() - 1].len() - 1) {
            if data[i][j] == 'A' {
                let diagOne : HashSet<char> = vec![  data[i-1][j-1], data[i+1][j+1]  ].into_iter().collect();//slash forward
                let diagTwo : HashSet<char> = vec![  data[i-1][j+1], data[i+1][j-1]  ].into_iter().collect();//slash backward

                if checkMap == diagOne && checkMap == diagTwo {
                    count += 1
                };
            };
        };
    };
    count
}


#[cfg(test)]
mod sample_case {
    use super::*;

    #[test]
    fn sample_one() {
        let input : String = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX".to_string();
        let check = 18i32;

        assert_eq!(one(input), check)
    }

    #[test]
    fn sample_two() {
        let input : String = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX".to_string();
        let check = 9i32;

        assert_eq!(two(input), check)
    }
}