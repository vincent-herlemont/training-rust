
fn chunk<T>(mut v: Vec<T>, i: usize) -> Vec<Vec<T>> {
    let mut out = vec![];
    while !v.is_empty() {
        let l = v.len();
        let i = if i > l {
            l
        } else {
            i
        };
        let chunk: Vec<_> = v.drain(0..i).collect();
        out.push(chunk);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_chunk(){
        assert_eq!(chunk(vec![1,2,3,4],2),vec![vec![1,2],vec![3,4]]);
        assert_eq!(chunk(vec![1,2,3,4,5],2),vec![vec![1,2],vec![3,4],vec![5]]);
        assert_eq!(chunk(vec![1,2,3,4,5],4),vec![vec![1,2,3,4],vec![5]]);
        assert_eq!(chunk(vec![1,2,3,4,5],10),vec![vec![1,2,3,4,5]])
    }
}