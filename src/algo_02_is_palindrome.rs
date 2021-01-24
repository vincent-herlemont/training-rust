use super::algo_01_string_reversal::reverse;

// Return true if str is a palindrome.
fn is_palindrome(str: &str) -> bool {
    let rev_str = reverse(str);
    str == rev_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("abba"));
    }

    fn test_is_not_palindrome() {
        assert!(!is_palindrome("palindrome"));
    }
}