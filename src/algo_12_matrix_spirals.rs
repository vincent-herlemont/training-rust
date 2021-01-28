fn matrix_spirals(size: usize) -> Vec<Vec<usize>> {
    let mut matrix = vec![vec![0;size];size];
    let mut i_top_line = 0;
    let mut i_right_column = size;
    let mut i_bottom_line = size;
    let mut i_left_column = 0;
    let mut i = 1;

    while i_left_column < i_right_column {
        // Top line
        for n in i_left_column..i_right_column {
            matrix[i_top_line][n] = i;
            i += 1;
        }
        i_top_line += 1;

        // Right column
        for n in i_top_line..i_bottom_line {
            matrix[n][i_right_column - 1] = i;
            i += 1;
        }
        i_right_column -= 1;

        // Bottom line
        for n in (i_left_column..i_right_column).rev() {
            matrix[i_bottom_line - 1][n] = i;
            i += 1;
        }
        i_bottom_line -= 1;

        // Left column
        for n in (i_top_line..i_bottom_line).rev() {
            matrix[n][i_left_column] = i;
            i += 1;
        }
        i_left_column += 1;
    }

    matrix
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_matrix_spirals(){
        let expect :Vec<Vec<usize>> = vec![
            vec![1,2,3],
            vec![8,9,4],
            vec![7,6,5]
        ];
        assert_eq!(matrix_spirals(3),expect);
    }


    // #[test]
    fn test_print() {
        let matrix = matrix_spirals(10);
        for line in matrix {
            for n in line {
                print!("{:^5}",n)
            }
            print!("\n")
        }
    }
}