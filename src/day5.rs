use std::collections::HashSet;

pub fn one(input: String) -> i32 {
    //does not handle CRLF line breaks
    let instr : Vec<Vec<i32>> = input.split("\n\n").next().unwrap().split("\n").map(|x| x.split("|").map(|y| y.parse::<i32>().unwrap() ).collect::<Vec<_>>() ).collect::<Vec<_>>();
    let data : Vec<Vec<i32>> = input.split("\n\n").nth(1).unwrap().split("\n").map(|x| x.split(",").map(|y| y.parse::<i32>().unwrap() ).collect::<Vec<_>>() ).collect::<Vec<_>>();
    let mut count = 0i32;

    'data : for i in 0..data.len() {
        let dataSet : HashSet<i32> = data[i].clone().into_iter().collect();
        let mut instrSet : Vec<Vec<i32>> = Vec::new();

        for j in 0..instr.len() {
            let rule : HashSet<i32> = instr[j].clone().into_iter().collect();
            if dataSet.intersection(&rule).collect::<Vec<_>>().len() == 2 {
                instrSet.push(instr[j].clone());
            };
        };

        'instr : for j in 0..instrSet.len() {
            for k in 0..data[i].len() {
                if data[i][k] == instrSet[j][0] {
                    continue 'instr;
                } else if data[i][k] == instrSet[j][1] {
                    continue 'data;
                } else {
                    continue;
                };
            };
        };

        count += data[i][data[i].len() / 2];
    };
    count
}

pub fn two(input: String) -> i32 {
    //does not handle CRLF line breaks
    let instr : Vec<Vec<i32>> = input.split("\n\n").next().unwrap().split("\n").map(|x| x.split("|").map(|y| y.parse::<i32>().unwrap() ).collect::<Vec<_>>() ).collect::<Vec<_>>();
    let data : Vec<Vec<i32>> = input.split("\n\n").nth(1).unwrap().split("\n").map(|x| x.split(",").map(|y| y.parse::<i32>().unwrap() ).collect::<Vec<_>>() ).collect::<Vec<_>>();
    let mut count = 0i32;

    'data : for i in 0..data.len() {
        let dataSet : HashSet<i32> = data[i].clone().into_iter().collect();
        let mut instrSet : Vec<Vec<i32>> = Vec::new();

        for j in 0..instr.len() {
            let rule : HashSet<i32> = instr[j].clone().into_iter().collect();
            if dataSet.intersection(&rule).collect::<Vec<_>>().len() == 2 {
                instrSet.push(instr[j].clone());
            };
        };
        // println!("{:?}", dataSet);
        // println!("{:?}", instrSet);

        'instr : for j in 0..instrSet.len() {
            for k in 0..data[i].len() {
                if data[i][k] == instrSet[j][0] {
                    continue 'instr;
                } else if data[i][k] == instrSet[j][1] {
                    println!("{:?}", data[i]);
                    let mut fixedSet = data[i].clone();
                    fixedSet = sort_list(fixedSet, instrSet.clone());
                    count += fixedSet[fixedSet.len() / 2];
                    continue 'data;
                } else {
                    continue;
                };
            };
        };
    };
    count
}

fn swap(list : &mut Vec<i32>, i : usize, j :usize) -> () {
    let hold = list[i];
    list[i] = list[j];
    list[j] = hold;
}

fn sort_list(mut list : Vec<i32>, rules : Vec<Vec<i32>>) -> Vec<i32> {
    'rules : for i in 0..rules.len() {
        for j in 0..list.len() {
            if list[j] == rules[i][0] {
                continue 'rules;
            } else if list[j] == rules[i][1] {
                let indexOne = j;
                for k in 0..list.len() {
                    if list[k] == rules[i][0] {
                        let indexTwo = k;
                        swap(&mut list, indexOne, indexTwo);
                        return sort_list(list, rules);
                    };
                };
            };
        };
    };
    list
}


#[cfg(test)]
mod sample_case {
    use super::*;

    #[test]
    fn sample_one() {
        let input : String = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47".to_string();
        let check = 143i32;

        assert_eq!(one(input), check)
    }

    #[test]
    fn sample_two() {
        let input : String = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47".to_string();
        let check = 123i32;

        assert_eq!(two(input), check)
    }
}