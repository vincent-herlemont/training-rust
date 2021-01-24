use super::algo_01_string_reversal::reverse;

// Return an reversed integer example
// 321 123
// -321 -123
fn reverse_integer(n: i32) -> i32 {
    let neg = n < 0;

    let n_str = if neg {
        (n.abs()).to_string()
    } else {
        n.to_string()
    };

    let rev_n_str = reverse(&n_str);

    if neg {
        rev_n_str.parse::<i32>().unwrap() * -1
    } else {
        rev_n_str.parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_integer() {
        assert_eq!(reverse_integer(321),123);
        assert_eq!(reverse_integer(-321),-123);
    }
}