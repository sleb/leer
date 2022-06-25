use std::collections::HashMap;

const CHARS: [(char, i32); 10] = [
    ('0', 0),
    ('1', 1),
    ('2', 2),
    ('3', 3),
    ('4', 4),
    ('5', 5),
    ('6', 6),
    ('7', 7),
    ('8', 8),
    ('9', 9),
];

pub fn my_atoi(s: String) -> i32 {
    let ch_to_i = HashMap::from(CHARS);
    let mut digits: Vec<i32> = Vec::with_capacity(s.len());
    let mut leading = true;
    let mut sign = 1;
    for ch in s.chars() {
        match (ch, leading) {
            ('-', true) => {
                sign = -1;
                leading = false;
            }
            ('+', true) => {
                leading = false;
            }
            (' ', true) => continue,
            (' ', false) => break,
            (c, _) => {
                leading = false;
                if let Some(&i) = ch_to_i.get(&c) {
                    digits.push(i);
                } else {
                    break;
                }
            }
        }
    }

    let mut result: i32 = 0;
    for digit in digits {
        if let (prod, false) = result.overflowing_mul(10) {
            if let (sum, false) = prod.overflowing_add(digit) {
                result = sum;
            } else {
                return overflow(sign);
            }
        } else {
            return overflow(sign);
        }
    }

    result * sign
}

fn overflow(sign: i32) -> i32 {
    if sign < 0 {
        return i32::MIN;
    } else {
        return i32::MAX;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(42, my_atoi(String::from("42")));
    }

    #[test]
    fn example2() {
        assert_eq!(-42, my_atoi(String::from("-42")));
    }

    #[test]
    fn example3() {
        assert_eq!(4193, my_atoi(String::from("4193 with words")));
    }

    #[test]
    fn example4() {
        assert_eq!(0, my_atoi(String::from("+-12")));
    }

    #[test]
    fn example5() {
        assert_eq!(2147483647, my_atoi(String::from("2147483648")));
    }
}
