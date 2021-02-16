
fn integer_reversal(i: &i32) -> i32 {
    let str: String = i.abs().to_string().chars().rev().collect();
    let out: i32 = str.parse().unwrap();
    if i < &0 {
        -out
    } else {
        out
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_reversal() {
        assert_eq!(integer_reversal(&123),321);
    }

    #[test]
    fn test_integer_reversal_neg() {
        assert_eq!(integer_reversal(&-123),-321);
    }
}