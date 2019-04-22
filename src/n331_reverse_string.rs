pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        s.reverse();
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut test_input_1 = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut test_input_1);
        assert_eq!(vec!['o', 'l', 'l', 'e', 'h'], test_input_1);
    }
}
