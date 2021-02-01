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
    // s == aXXcdefga̐
    println!("{}",s);
}

fn declaration_array_slice() {
    let a: [i32;3] = [1,2,3];  // array
    let s: &[i32] = &[1,2,3];  // slice of array
    let v = vec![1,2,3]; // Vec<i32>

    let mut a = a;
    {
        // Update by slice (array reference)
        // Create slice from array
        let x = &mut a[..];
        // We can updated an element
        x[0] = 7;
        println!("{:?}", x);
        // x == [7, 2, 3]
    }
    // ---------------- OR ------------------
    {
        // Update by element reference
        // A reference is created for each elements.
        let mut x: Vec<&mut i32> = a.iter_mut().collect();
        // We can add element but is only available for 'x'.
        let v = &mut 9;
        x.push(v);
        // We can updated element
        *x[1] = 8;
        println!("{:?}",x);
        // x == [7, 8, 3, 9]
    }
    // a == [7, 8, 3]
    println!("{:?}",a);
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

    #[test]
    fn test_declaration_array_slice() {
        declaration_array_slice();
    }
}