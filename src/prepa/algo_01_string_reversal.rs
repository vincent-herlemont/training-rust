fn string_reversal(str: &str) -> String {
    str.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_reversal() {
        assert_eq!(string_reversal("apple"),"elppa");
    }
}