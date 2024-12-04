pub fn one(input: String) -> i32 {

}

pub fn two(input: String) -> i32 {

}


#[cfg(test)]
mod sample_case {
    use super::*;

    #[test]
    fn sample_one() {
        let input : String = "".to_string();
        let check = 0i32;

        assert_eq!(one(input), check)
    }

    #[test]
    fn sample_two() {
        let input : String = "".to_string();
        let check = 0i32;

        assert_eq!(two(input), check)
    }
}