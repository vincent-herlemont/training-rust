// print_steps(3)
// #
// ##
// ###

pub fn print_canvas(canvas:&Vec<Vec<char>>) -> String {
    canvas.iter().map(|line| {
        let mut line: String = line.iter().collect();
        line.push('\n');
        line
    }).collect()
}

fn print_steps(n: usize) -> String {
    // Create vector
    let mut canvas = vec![];
    for il in 0..n {
        let mut line = vec![' ';3];
        for is in 0..il + 1  {
            line[is] = '#';
        }
        canvas.push(line);
    }

    // Create string from vec of char
    print_canvas(&canvas)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_steps(){
        let expect = "#  \n## \n###\n";
        assert_eq!(print_steps(3),expect);
    }
}