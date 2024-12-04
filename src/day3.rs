pub fn one(input: String) -> i32 {
    let dataOne : Vec<&str> = input.split("\n").collect();
    let mut value : i32 = 0;

    for memRow in dataOne {
        let memRowBytes = memRow.as_bytes();
        let mulMatches : Vec<(usize, &str)> = memRow.match_indices("mul(").collect();

        //we require acsii encoding so we check lens match
        //doing so guarantees the indecies in mulMatches is valid
        //for memRow and memRowBytes
        assert_eq!(memRow.len(), memRowBytes.len());

        for m in mulMatches {
            let indexS = m.0;
            let mut indexE = 0usize;

            for i in 7..=11 {
                let result = match memRowBytes.get(indexS + i) {
                    Some(x) => x,
                    None => &0u8
                };

                if *result == b')' {
                    indexE = indexS + i;
                    break
                };
            };

            //handles panic on out of bound attempt
            match memRow.get(indexS..=indexE) {
                Some(text) => {},
                None => continue
            }

            let text = memRow.get(indexS..=indexE).clone().unwrap();
            value += check_mul(text);
        };
    };
    value

}

pub fn two(input: String) -> i32 {
    let dataOne : Vec<&str> = input.split("\n").collect();
    let mut mulActive : bool = true;
    let mut value : i32 = 0;

    for memRow in dataOne {
        let memRowBytes = memRow.as_bytes();
        let mut mulMatches : Vec<(usize, &str)> = memRow.match_indices("mul(").collect();
        let mut doMatches : Vec<(usize, &str)> = memRow.match_indices("do()").collect();
        let mut dontMatches : Vec<(usize, &str)> = memRow.match_indices("don't()").collect();

        let mut lineCommands : Vec<(usize, &str)> = order_commands(doMatches, dontMatches);
        lineCommands = order_commands(mulMatches, lineCommands);


        //we require acsii encoding so we check lens match
        //doing so guarantees the indecies in mulMatches is valid
        //for memRow and memRowBytes
        assert_eq!(memRow.len(), memRowBytes.len());

        for cmd in lineCommands {
            match cmd.1 {
                "do()" => {mulActive = true;},
                "don't()" => {mulActive = false;},
                "mul(" => {
                    let indexS = cmd.0;
                    let mut indexE = 0usize;

                    for i in 7..=11 {
                        let result = match memRowBytes.get(indexS + i) {
                            Some(x) => x,
                            None => &0u8
                        };

                        if *result == b')' {
                            indexE = indexS + i;
                            break
                        };
                    };

                    //handles panic on out of bound attempt
                    match memRow.get(indexS..=indexE) {
                        Some(text) => {},
                        None => continue
                    }

                    let text = memRow.get(indexS..=indexE).clone().unwrap();
                    value += check_mul(text) * i32::from(mulActive);
                },
                _ => {}
            };
        };
    };
    value
}

fn check_mul(text : &str) -> i32 {
    println!("{text}");
    if text.contains(",") == false {
        return 0
    };

    let mut x = 0i32;
    let mut y = 0i32;

    //check first argument - mul(|xxx|,xxx)
    let mut textFirst = text.split_once('(').unwrap().1; //mul | xxx,xxx)
    textFirst = textFirst.split_once(',').unwrap().0; //xxx | xxx)

    if textFirst.matches(char::is_numeric).collect::<Vec<_>>().len() == textFirst.len() {
        x = match textFirst.parse::<i32>() {
            Ok(num) => num, 
            Err(e) => 0
        };
    };

    //check second argument - mul(xxx,|xxx|)
    let mut textSecond = text.split_once(',').unwrap().1; //mul(xxx | xxx)
    textSecond = textSecond.split_once(')').unwrap().0;//xxx |

    if textSecond.matches(char::is_numeric).collect::<Vec<_>>().len() == textSecond.len() {
        y = match textSecond.parse::<i32>() {
            Ok(num) => num,
            Err(e) => 0
        };
    };

    x * y
}

fn order_commands<'a>(mut listA: Vec<(usize, &'a str)>, mut listB: Vec<(usize, &'a str)>) -> Vec<(usize, &'a str)> {
    let mut output : Vec<(usize, &str)> = Vec::new();
    let n = listA.len() + listB.len();

    for i in 0..n {

        if listA.len() > 0 && listB.len() > 0 {

            if listA[0].0 > listB[0].0 {
                output.push(listB.remove(0));
            } else if listA[0].0 < listB[0].0 {
                output.push(listA.remove(0));
            } else {
                panic!();
            };
            
        } else if listA.len() > 0 && listB.len() == 0 {
            output.append(&mut listA);
            break;
        } else if listA.len() == 0 && listB.len() > 0 {
            output.append(&mut listB);
            break;
        } else {
            continue
        };
    };
    output
}


#[cfg(test)]
mod sample_case {
    use super::*;

    #[test]
    fn sample_one() {
        let input : String = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
        let check = 161i32;

        assert_eq!(one(input), check)
    }

    #[test]
    fn sample_two() {
        let input : String = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
        let check = 48i32;

        assert_eq!(two(input), check)
    }
}