use std::{cmp, collections::HashSet};

pub fn length_of_longest_substring(s: String) -> i32 {
    let s: Vec<char> = s.chars().collect();
    let mut start: usize = 0;
    let mut end = start;
    let mut max = 0;
    let mut chars = HashSet::new();
    while start <= end && end < s.len() && s.len() - start > max {
        if chars.insert(s[end]) {
            end += 1;
            max = cmp::max(max, end - start);
        } else {
            chars.remove(&s[start]);
            start += 1;
        }
    }

    max as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let s = String::from("abcabcbb");
        assert_eq!(3, length_of_longest_substring(s));
    }

    #[test]
    fn example2() {
        let s = String::from("bbbbb");
        assert_eq!(1, length_of_longest_substring(s));
    }

    #[test]
    fn example3() {
        let s = String::from("pwwkew");
        assert_eq!(3, length_of_longest_substring(s));
    }

    #[test]
    fn example4() {
        let s = String::from(" ");
        assert_eq!(1, length_of_longest_substring(s));
    }
}
