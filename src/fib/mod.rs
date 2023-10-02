use std::collections::HashMap;

pub fn fib(n: u32) -> u32 {
    let mut cache: HashMap<u32, u32> = HashMap::from([
        (0, 0),
        (1, 1)
    ]);

    if n == 0 { return 0 }
    if n == 1 { return 1 }
    
    *cache.entry(n).or_insert_with(|| fib(n - 1) + fib(n - 2))
}