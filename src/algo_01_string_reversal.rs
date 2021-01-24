
// Return the reverse string
pub fn reverse(str: &str) -> String {
    str.chars().into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("apple"),"elppa");
    }
}