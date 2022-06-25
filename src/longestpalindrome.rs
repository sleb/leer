struct Bounds {
    start: usize,
    end: usize,
}

impl Bounds {
    fn len(&self) -> usize {
        self.end - self.start
    }
}

pub fn longest_palindrome(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut start = 0;
    let mut end = 0;
    let mut longest = Bounds { start, end };
    while end < s.len() {
        if chars[start] == chars[end] {
            let current_longest = expand_out(start, end, &chars);
            if current_longest.len() > longest.len() {
                longest = current_longest;
            }
        }

        if start == end {
            end += 1;
        } else {
            start += 1;
        }
    }

    chars[longest.start..=longest.end].iter().collect()
}

fn expand_out(start: usize, end: usize, chars: &Vec<char>) -> Bounds {
    let mut bounds = Bounds { start, end };
    let mut l = start as i32;
    let mut r = end as i32;

    while l >= 0 && r < chars.len() as i32 && chars[l as usize] == chars[r as usize] {
        if (r - l) as usize > bounds.len() {
            bounds = Bounds {
                start: l as usize,
                end: r as usize,
            };
        }

        l -= 1;
        r += 1;
    }

    bounds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            String::from("bab"),
            longest_palindrome(String::from("babad"))
        );
    }

    #[test]
    fn example2() {
        assert_eq!(String::from("bb"), longest_palindrome(String::from("cbbd")));
    }

    #[test]
    fn example3() {
        assert_eq!(
            String::from("aaabaaa"),
            longest_palindrome(String::from("aaabaaaa"))
        );
    }
}
