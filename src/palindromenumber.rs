pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut digits = Vec::new();
    let mut n = x;
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    is_palindrome_digits(&digits)
}

fn is_palindrome_digits(digits: &Vec<i32>) -> bool {
    if digits.len() <= 1 {
        return true;
    }

    let mut i = 0;
    let mut j = digits.len() - 1;

    while i < j {
        if digits[i] != digits[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert!(is_palindrome(121));
    }

    #[test]
    fn example2() {
        assert!(!is_palindrome(-121));
    }

    #[test]
    fn example3() {
        assert!(!is_palindrome(10));
    }

    #[test]
    fn example4() {
        assert!(is_palindrome(0));
    }
}
