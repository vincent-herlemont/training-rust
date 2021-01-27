// print_pyramids(4)
//    #      (3+1)
//   ###     (2+3)
//  #####    (1+5)
// #######   (0+7)
// 1 -> 1  1+0
// 2 -> 3  2+1
// 3 -> 5  3+2
// 4 -> 7  4+3
// nc = n+(n-1)

use crate::algo_09_print_steps::print_canvas;

fn print_pyramids(n: usize) -> String {
    let height: isize = n as isize;
    let width = height + (height - 1);

    let mut canvas = vec![];

    for i_line in 0..height {
        let i_start_char = width / 2 - i_line;
        let nb_char = width - i_start_char * 2;
        let mut line = vec![' '; width as usize];
        for i_char in i_start_char..i_start_char + nb_char {
            line[i_char as usize] = '#'
        }
        canvas.push(line);
    }

    print_canvas(&canvas)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_print_pyramids() {
        let expected = "#\n";
        assert_eq!(print_pyramids(1),expected);
        let expected = " # \n###\n";
        assert_eq!(print_pyramids(2),expected);
        let expected = "  #  \n ### \n#####\n";
        assert_eq!(print_pyramids(3),expected);
        let expected = "   #   \n  ###  \n ##### \n#######\n";
        assert_eq!(print_pyramids(4),expected);
    }
}