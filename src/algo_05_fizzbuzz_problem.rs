use std::fmt::Write as FmtWrite;

// Return string to increment from 1 to n.
// But multiples of three print "fizz" and
// multiples of five print buzz "buzz".
fn fizzbuzz(n: i32) -> String {
    let mut buf = String::new();
    for n in 1..n+1 {
        match n {
            n if n % 3 == 0 => writeln!(&mut buf, "fizz").unwrap(),
            n if n % 5 == 0 => writeln!(&mut buf, "buzz").unwrap(),
            n => writeln!(&mut buf, "{}", n).unwrap()
        };
    }

    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz(){
        let expected = r"1
2
fizz
4
buzz
fizz
7
";
        assert_eq!(fizzbuzz(7), expected)
    }
}