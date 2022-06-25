pub fn reverse(x: i32) -> i32 {
    let sign = if x > 0 { 1 } else { -1 };
    let mut n = x.abs();
    let mut result: i32 = 0;
    while n > 0 {
        let digit = n % 10;
        if let (val, false) = result.overflowing_mul(10) {
            result = val + digit;
        } else {
            return 0;
        }

        n /= 10
    }

    result * sign
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(321, reverse(123));
    }

    #[test]
    fn example2() {
        assert_eq!(-321, reverse(-123));
    }

    #[test]
    fn example3() {
        assert_eq!(21, reverse(120));
    }

    #[test]
    fn example4() {
        assert_eq!(0, reverse(1534236469));
    }
}
