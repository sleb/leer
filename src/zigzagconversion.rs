pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }

    let bottom = num_rows - 1;
    let mut rows = vec![String::with_capacity(s.len()); num_rows as usize];

    let mut row = 0;
    let mut direction: i32 = 1;
    for c in s.chars() {
        rows[row].push(c);
        if row == 0 {
            direction = 1;
        } else if row == bottom as usize {
            direction = -1;
        }
        row = (row as i32 + direction) as usize;
    }

    rows.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            String::from("PAHNAPLSIIGYIR"),
            convert(String::from("PAYPALISHIRING"), 3)
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            String::from("PINALSIGYAHRPI"),
            convert(String::from("PAYPALISHIRING"), 4)
        );
    }

    #[test]
    fn example3() {
        assert_eq!(String::from("AB"), convert(String::from("AB"), 1));
    }
}
