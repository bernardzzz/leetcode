pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut symbol_stack: Vec<char> = Vec::new();
        const REMOVE_LEVEL: i32 = 1;
        const OPEN_SYMBOL: char = '(';
        let mut current_level = 0;
        for current_symbol in s.chars() {
            if current_symbol == OPEN_SYMBOL {
                current_level += 1;
                if current_level > REMOVE_LEVEL {
                    symbol_stack.push(current_symbol);
                }
            } else {
                if current_level > REMOVE_LEVEL {
                    symbol_stack.push(current_symbol);
                }
                current_level -= 1;
            }
        }
        symbol_stack.iter().collect::<String>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "()()()",
            Solution::remove_outer_parentheses("(()())(())".to_string())
        );
        assert_eq!(
            "()()()()(())",
            Solution::remove_outer_parentheses("(()())(())(()(()))".to_string())
        );
        assert_eq!("", Solution::remove_outer_parentheses("()()".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "()()()",
            Solution::remove_outer_parentheses("(()())(())".to_string())
        );
    }
}
