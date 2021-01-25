// Return the sentence with each words beginning with capital.
use unicode_segmentation::UnicodeSegmentation;

fn sentence_capitalization(str: &str) -> String {
    let mut out = str.to_owned();
    for (i,words) in str.split_word_bound_indices() {
        if let Some(grapheme_cluster) = words.graphemes(true).next() {
            let grapheme_cluster = grapheme_cluster.to_uppercase();
            out.replace_range(i..i + grapheme_cluster.len(), &grapheme_cluster);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sentence_capitalization() {
        assert_eq!(sentence_capitalization("a short sentence"), "A Short Sentence");
        assert_eq!(sentence_capitalization("a lazy fox"), "A Lazy Fox");
        assert_eq!(sentence_capitalization("look, it is working!"), "Look, It Is Working!");
    }
}