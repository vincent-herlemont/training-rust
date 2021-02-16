use std::collections::HashMap;

fn max_char(str: &str) -> Option<char> {
    let mut char_map = HashMap::<char,i32>::new();
    for c in str.chars() {
        if let Some(i) = char_map.get_mut(&c) {
            *i += 1;
        } else {
            char_map.insert(c,1);
        }
    }

    let mut iter_char_map = char_map.into_iter();
    let mut out= iter_char_map.next()?;
    for ci in iter_char_map {
        out = if ci.1 > out.1 {
            ci
        } else {
            out
        }
    }

    Some(out.0)
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