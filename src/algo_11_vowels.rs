use unicode_segmentation::UnicodeSegmentation;

fn vowels(str: &str) -> usize {
    let mut count = 0;
    for grapheme  in str.graphemes(true) {
        match grapheme {
            "a" | "o" | "i" | "e" | "u" => count += 1,
            _ => ()
        }
    }
    count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vowels(){
        assert_eq!(vowels("Hi there!"),3);
        assert_eq!(vowels("Why do you ask?"),4);
        assert_eq!(vowels("Why?"),0);
    }
}