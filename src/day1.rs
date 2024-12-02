pub fn one(input: String) -> i32 {
    let data1 : Vec<&str> = input.split("\n").collect();
    let mut data2 : Vec<i32> = Vec::new();

    for i in data1 {
        let mut temp = i.split_whitespace();
        data2.push(  temp.next().unwrap().parse::<i32>().unwrap()  );
        data2.push(  temp.next().unwrap().parse::<i32>().unwrap()  );
    }

    let mut listOne : Vec<i32> = data2.iter().step_by(2).copied().collect();
    let mut listTwo : Vec<i32> = data2.iter().skip(1).step_by(2).copied().collect();

    listOne.sort();
    listTwo.sort();

    //check equal len
    assert_eq!(listOne.len(), listTwo.len());

    let n = listOne.len();
    let mut value : i32 = 0;

    for i in 0..n {
        value += (listOne[i] - listTwo[i]).abs();
    }

    value
}

pub fn two(input: String) -> i32 {
    let data1 : Vec<&str> = input.split("\n").collect();
    let mut data2 : Vec<i32> = Vec::new();

    for i in data1 {
        let mut temp = i.split_whitespace();
        data2.push(  temp.next().unwrap().parse::<i32>().unwrap()  );
        data2.push(  temp.next().unwrap().parse::<i32>().unwrap()  );
    }

    let mut listOne : Vec<i32> = data2.iter().step_by(2).copied().collect();
    let mut listTwo : Vec<i32> = data2.iter().skip(1).step_by(2).copied().collect();

    listOne.sort();
    listTwo.sort();

    let n = listOne.len();
    let mut coefs : Vec<i32> = Vec::new();

    for i in 0..n {
        
        let mut count : i32 = 0;
        for j in 0..n {
            if listOne[i] == listTwo[j] {
                count += 1;
                continue
            } else if listOne[i] < listTwo[j] {
                coefs.push(count);
                break
            } else if listOne[i] > listTwo[j] && j == 999 {
                coefs.push(0);
            }
        }
    }

    //check lens
    assert_eq!(coefs.len(), listOne.len());

    // println!("{:?}", coefs);
    // println!("{:?}", listOne);

    listOne.iter().zip(coefs.iter()).map(|(x, y)| x * y).sum()

}


#[cfg(test)]
mod sample_case {
    use super::*;

    #[test]
    fn sample_one() {
        let input : String = "3   4
4   3
2   5
1   3
3   9
3   3".to_string();
        let check : i32 = 11;

        assert_eq!(one(input), check)
    }

    #[test]
    fn sample_two() {
        let input : String = "3   4
4   3
2   5
1   3
3   9
3   3".to_string();
        let check : i32 = 31;

        assert_eq!(two(input), check)
    }
}