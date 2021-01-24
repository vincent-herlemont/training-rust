// Given an array and chunk size.
// Return an array containing many subarrays
// where each subarray is of length chunk size.
// Note : subarrays are chunks.
fn chunk<T>(v: Vec<T>,size: usize) -> Vec<Vec<T>> {
    let mut out = vec![];
    let mut chunk = vec![];
    for (i,v) in  v.into_iter().enumerate() {
        chunk.push(v);
        if (i + 1) % size == 0 {
            out.push(chunk.drain(..).collect());
            chunk = vec![];
        }
    }
    if !chunk.is_empty() {
        out.push(chunk.drain(..).collect());
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