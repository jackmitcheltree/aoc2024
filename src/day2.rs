pub fn one(input: String) {

}

pub fn two(input: String) {

}


#[cfg(test)]
mod sample_case {
    use super::*;

    #[test]
    fn sample_one() {
        let input : String = "".to_string();
        let check = "";

        assert_eq!(one(input), check)
    }

    #[test]
    fn sample_two() {
        let input : String = "".to_string();
        let check = "";

        assert_eq!(two(input), check)
    }
}