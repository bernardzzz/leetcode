pub struct Solution {}

// submission codes start here


impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        use std::collections::HashSet;
        let histo : HashSet<char> = j.chars().collect();
        s.chars()
            .filter(|c| histo.contains(c))
            .count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::num_jewels_in_stones(String::from("aA"), String::from("aAAbbbb")));
        assert_eq!(0, Solution::num_jewels_in_stones(String::from("z"), String::from("ZZ")));
    }
}