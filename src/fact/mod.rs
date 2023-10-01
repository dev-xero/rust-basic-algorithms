pub fn fact(n: u32) -> u32 {
    if n == 1 {
        return n;
    }

    n * fact(n - 1)
}