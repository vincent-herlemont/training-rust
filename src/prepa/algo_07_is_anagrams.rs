use std::collections::HashMap;

fn is_anagrams(str_a: &str, str_b: &str) -> bool {
    let char_map = |str: &str| {
        let mut char_map = HashMap::<char,i32>::new();
        for c in str.chars() {
            if let Some(i) = char_map.get_mut(&c) {
                *i += 1;
            } else {
                char_map.insert(c,1);
            }
        }
        char_map
    };

    char_map(str_a) == char_map(str_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagrams(){
        assert!(is_anagrams("apple","ppale"));
        assert!(!is_anagrams("apple","test"));
    }
}