use std::fmt::Write as FmtWrite;

fn fizzbuzz(i: u32) -> String {
    let mut out = String::new();
    for n in 1..i+1 {
        match n {
            n if n % 3 == 0 => writeln!(&mut out,"fizz").unwrap(),
            n if n % 5 == 0 => writeln!(&mut out,"buzz").unwrap(),
            n => writeln!(&mut out,"{}",n).unwrap(),
        }
    }
    out
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