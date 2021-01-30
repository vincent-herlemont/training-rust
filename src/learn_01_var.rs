use unicode_segmentation::UnicodeSegmentation;

fn declaration_int() {
    let a = 1; // Default type is i32
    let a:i32 = 1;
}

fn update_var() {
    let mut a = 1;
    a += 1; // a == 2
    println!("{}",a);
}

fn declaration_string() {
    let s: &'static str  = "abcdefga̐";  // String slice
    let s = String::from(s);
    let slice_1 = &s[0..4]; // abcd
    let slice_2  = s.get(1..).unwrap(); // bcdefga̐
    println!("{}",slice_1);
    println!("{}",slice_2);

    let mut chars: Vec<char> = s.chars().collect();
    // chars == ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'a', '\u{310}']
    println!("{:?}",chars);
    let s_from_chars: String = chars.iter().collect();
    println!("{}", s_from_chars); // bcdefga̐

    let mut graphemes:Vec<&str> = s.graphemes(true).collect();
    // graphemes == ["a", "b", "c", "d", "e", "f", "g", "a\u{310}"]
    println!("{:?}", graphemes);
    let s_from_graphemes: String = graphemes.into_iter().collect();
    println!("{}", s_from_graphemes); // bcdefga̐

    let mut s = s;
    s.replace_range(1..2,"XX");
    println!("{}",s);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_declaration_int(){
        declaration_int();
    }

    #[test]
    fn test_declaration_string(){
        declaration_string();
    }

    #[test]
    fn test_update_var() {
        update_var();
    }
}