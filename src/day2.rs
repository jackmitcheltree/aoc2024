pub fn one(input: String) -> i32 {
    let dataOne : Vec<&str> = input.split("\n").collect();
    let mut dataTwo : Vec<Vec<i32>> = Vec::new();

    for i in dataOne {
        let temp : Vec<i32> = i.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        dataTwo.push(temp);
    };

    let minStep = 1i32;
    let maxStep = 3i32;
    let mut count = 0i32;

    'reports : for report in dataTwo {
        let rlen : usize = report.len();
        let s : i32 = (report[1] - report[0]).signum();
        if s == 0 {
            continue
        };

        'levels : for i in 0..(rlen-1) {
            let diff : i32 = s * (report[i+1] - report[i]);
            if diff <= 0 || 3 < diff {
                continue 'reports;
            };
        };
        // println!("{:?}", report);
        count += 1;

    };

    count
}

pub fn two(input: String) -> i32 {
    let dataOne : Vec<&str> = input.split("\n").collect();
    let mut dataTwo : Vec<Vec<i32>> = Vec::new();

    for i in dataOne {
        let temp : Vec<i32> = i.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        dataTwo.push(temp);
    };

    let minStep = 1i32;
    let maxStep = 3i32;
    let mut count = 0i32;

    for mut report in dataTwo {
        let rlen : usize = report.len();
        let s : i32 = (report[1] - report[0]).signum();

        for i in 0..(rlen-1) {
            let diff : i32 = report[i+1] - report[i];

            if s == diff.signum() && minStep <= diff.abs() && diff.abs() <= maxStep {
                if i == rlen-2 {
                    count += 1
                };
                continue
            };

            let mut report_c = report.clone();
            let mut report_n = report.clone();
            let mut report_p = report.clone();

            report_c.remove(i);
            report_n.remove(i+1);

            println!("{i} {:?}",report_c);
            println!("{i} {:?}",report_n);

            match simple_check(report_c) {
                true => {count += 1; break;},
                false => {}
            };

            match simple_check(report_n) {
                true => {count += 1; break;},
                false => {}
            };

            if i > 0 {
                report_p.remove(i-1);
                println!("{i} {:?}", report_p);
                match simple_check(report_p) {
                    true => {count += 1; break;},
                    false => {}
                };
            };

            println!("{:?} Failed", report);
            break
        };
    };

    count
}

fn simple_check(report : Vec<i32>) -> bool {
    let mut value = true;
    let rlen : usize = report.len();
    let s : i32 = (report[1] - report[0]).signum();

    for i in 0..(rlen-1) {
        let diff : i32 = s * (report[i+1] - report[i]);
        if diff <= 0 || 3 < diff {
            value = false;
            break;
        };
    };

    value
}


#[cfg(test)]
mod sample_case {
    use super::*;

    #[test]
    fn sample_one() {
        let input : String = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9".to_string();
        let check : i32 = 2;

        assert_eq!(one(input), check)
    }

    #[test]
    fn sample_two() {
        let input : String = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
1 2 3 4 5 5
5 6 4 3 2
5 5 6 7 8
1 6 4 5 6
1 2 3 4 5 10
1 2 3 4 7 10 1
51 50 51 52 54".to_string();
let check : i32 = 11;

        assert_eq!(two(input), check)
    }
}