use std::collections::HashMap;

// Return the most used chars of a string.
fn max_char(str: &str) -> Option<char> {
    let mut char_map = HashMap::new();
    for c in str.chars() {
        if let Some(n) = char_map.get_mut(&c) {
            *n += 1;
        } else {
            char_map.insert(c,1);
        }
    }

    let mut iter = char_map.iter();
    let mut max = iter.next()?;
    for current in iter {
        if max.1 < current.1 {
            max = current;
        }
    }

    Some(*max.0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_char() {
        assert_eq!(max_char("apple"),Some('p'));
    }

    #[test]
    fn test_empty_max_char() {
        assert!(max_char(&"").is_none())
    }
}