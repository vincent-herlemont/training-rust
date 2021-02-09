// Fibonacci
// 1 1 2 3 5 8

use std::collections::HashMap;

fn fib_without_mem(n: u32) -> u32 {
    match n {
        0 => 0,
        1 | 2 => 1,
        n=> fib_without_mem(n - 1) + fib_without_mem(n - 2)
    }
}

fn fib_with_mem(n: u128,cache: &mut HashMap<u128,u128>) -> u128 {
    if let Some(cached_value) = cache.get(&n) {
        *cached_value
    } else {
        let value = match n {
            0 => 0,
            1 | 2 => 1,
            n => fib_with_mem(n-1, cache) + fib_with_mem(n-2, cache)
        };
        cache.insert(n, value);
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fib_without_mem() {
        assert_eq!(fib_without_mem(1),1);
        assert_eq!(fib_without_mem(5),5);
        assert_eq!(fib_without_mem(6),8);
    }

    #[test]
    fn test_fib_with_mem() {
        let mut cache = HashMap::new();
        assert_eq!(fib_with_mem(100,&mut cache),354224848179261915075);
    }
}