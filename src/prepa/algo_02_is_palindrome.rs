
fn is_palindrome(str: &str) -> bool {
    let rev: String = str.chars().rev().collect();
    str == rev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome(){
        assert!(is_palindrome("non"));
        assert!(!is_palindrome("apple"));
    }
}