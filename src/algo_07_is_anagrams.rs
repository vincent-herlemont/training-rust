use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

fn is_anagrams<'a>(str_a: &'a str, str_b:&'a str) -> bool {

    let grapheme_cluster_map = |str: &'a str| -> HashMap<&'a str,i32> {
        let mut grapheme_cluster_map = HashMap::new();
        for grapheme_cluster in str.graphemes(true) {
            if let Some(count) = grapheme_cluster_map.get_mut(&grapheme_cluster) {
                *count += 1;
            } else {
                grapheme_cluster_map.insert(grapheme_cluster, 1);
            }
        }
        grapheme_cluster_map
    };

    let grapheme_cluster_map_a = grapheme_cluster_map(str_a);
    let grapheme_cluster_map_b = grapheme_cluster_map(str_b);

    grapheme_cluster_map_a == grapheme_cluster_map_b
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