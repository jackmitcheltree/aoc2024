pub fn one(input: String) -> i64 {
    //does not handle CRLF endlines
    let checkValues : Vec<i64> = input.split("\n").map(|x| x.split(":").next().unwrap().parse::<i64>().unwrap()).collect();
    let data : Vec<Vec<i64>> = input.split("\n").map(|x| x.split(":").nth(1).unwrap().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()).collect();
    let mut result = 0i64;

    'datarow : for i in 0..data.len() {
        // println!("{:?}", bool_vec_permutations(data[i].len()- 1));
        // println!("{:?}", data[i]);

        for instr in bool_vec_permutations(data[i].len()- 1) {
            let mut value = data[i][0];

            for j in 0..(data[i].len()-1) {
                match instr[j] {
                    true => {value += data[i][j+1];},
                    false => {value *= data[i][j+1];}
                };
            };

            if value == checkValues[i] {
                result += value;
                continue 'datarow; 
            };
        }; 
    };
    result
}

pub fn two(input: String) -> i64 {
    //does not handle CRLF endlines
    let checkValues : Vec<i64> = input.split("\n").map(|x| x.split(":").next().unwrap().parse::<i64>().unwrap()).collect();
    let data : Vec<Vec<i64>> = input.split("\n").map(|x| x.split(":").nth(1).unwrap().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()).collect();
    let objs : Vec<i64> = vec![0,1,2];
    let mut result = 0i64;

    'datarow : for i in 0..data.len() {
        for instr in obj_vec_permutations(objs.clone(), data[i].len()- 1) {
            let mut value = data[i][0];

            for j in 0..(data[i].len()-1) {
                match instr[j] {
                    0 => {value += data[i][j+1];},
                    1 => {value *= data[i][j+1];},
                    2 => {value = value * 10i64.pow(data[i][j+1].ilog10() + 1) + data[i][j+1];},
                    _ => {},
                };
            };

            if value == checkValues[i] {
                result += value;
                continue 'datarow; 
            };
        }; 
    };
    result
}

pub fn two_fail(input: String) -> i64 {
    //does not handle CRLF endlines
    let checkValues : Vec<i64> = input.split("\n").map(|x| x.split(":").next().unwrap().parse::<i64>().unwrap()).collect();
    let data : Vec<Vec<i64>> = input.split("\n").map(|x| x.split(":").nth(1).unwrap().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()).collect();
    let mut result = 0i64;

    'datarow : for i in 0..data.len() {
        println!("data: {:?}", data[i] );

        for instrData in bool_vec_permutations(data[i].len()- 1) {
            let mut dataPerm : Vec<i64> = Vec::new();
            
            
            let mut value : i64 = 0;
            for j in 0..instrData.len() {
                if j == 0 {
                    value = data[i][0];
                };

                match instrData[j] {
                    true => {
                        value = value * 10i64.pow(data[i][j+1].ilog10() + 1) + data[i][j+1];
                        if j + 1 == instrData.len() {
                            dataPerm.push(value);
                        };
                    },
                    false => {
                        dataPerm.push(value);
                        value = data[i][j+1];
                        if j + 1 == instrData.len() {
                            dataPerm.push(value);
                        };
                    }
                };
            };
            // println!("data: {:?}", data[i] );
            // println!("{:?}", instrData );
            // println!("dtpm: {:?}", dataPerm );
            // println!("chk : {:?}", checkValues[i]  );

            let mut permValue = dataPerm[0];

            for instr in bool_vec_permutations(dataPerm.len()-1){
                permValue = dataPerm[0];
                println!("dtpm: {:?}", dataPerm );
                println!("inst: {:?}", instr  );
                println!("chk : {:?}", checkValues[i]  );

                for j in 0..(dataPerm.len()-1) {
                    match instr[j] {
                        true => {permValue += dataPerm[j+1];},
                        false => {permValue *= dataPerm[j+1];}
                    };
                };
                println!("pval: {:?}", permValue);
                // println!("____________");

                if permValue == checkValues[i] {
                    println!("added\n");
                    result += permValue;
                    continue 'datarow; 
                };
            };

            if permValue == checkValues[i] {
                println!("added\n");
                result += permValue;
                continue 'datarow; 
            };
        }; 
    };
    result
}

fn bool_vec_permutations(n : usize) -> Vec<Vec<bool>> {
    let mut result : Vec<Vec<bool>> = Vec::new();
    if n == 1 {
        return vec![vec![true], vec![false]];
    } else if n > 1 {
        let list = bool_vec_permutations(n-1);
        
        for subList in list {
            let mut subListTrue = subList.clone();
            subListTrue.insert(0, true);
            let mut subListFalse = subList.clone();
            subListFalse.insert(0, false);

            result.push(subListTrue);
            result.push(subListFalse);
        };
    };
    result
}

fn obj_vec_permutations<T : Clone>(objs : Vec<T>, n : usize) -> Vec<Vec<T>> {
    let mut result : Vec<Vec<T>> = Vec::new();
    if n == 1 {
        // return vec![vec![true], vec![false]];
        return objs.into_iter().map(|x| vec![x]).collect();
    } else if n > 1 {
        let list = obj_vec_permutations(objs.clone(), n-1);

        for i in objs {
            for j in 0..list.len() {
                let mut sublist = list[j].clone();
                sublist.insert(0, i.clone());
                result.push(sublist);
            };
        };
    };
    result
}


#[cfg(test)]
mod sample_case {
    use super::*;

    #[test]
    fn sample_one() {
        let input : String = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20".to_string();
        let check = 3749i64;

        assert_eq!(one(input), check)
    }

    #[test]
    fn sample_two() {
        let input : String = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20".to_string();
        let check = 11387i64;

        assert_eq!(two(input), check)
    }
}