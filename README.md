# Rust: Basic Algorithms
Basic algorithms implemented in Rust.

## Algorithms
1. Roots of a Quadratic Function
2. Factorial Function (recursive)
3. Fibonacci Sequence (dynamic programming)
4. Test for Primes

## Examples
### 1. Roots of a Quadratic Function / Equation
  Computes the roots of a valid quadratic equation.
```rust
  pub fn root(&self) -> (f64, f64) {
    let a = self.variables.0 as f64;
    let b = self.variables.1 as f64;
    let d = Self::discriminant(&self) as f64;

    if d >= 0.0 {
        let r1 = (-b + d.sqrt()) / (2.0 * a);
        let r2 = (-b - d.sqrt()) / (2.0 * a);

        (r1, r2)
    } else {
        let rp = -b / (2.0 * a);           // real part
        let ip = (-d).sqrt() / (2.0 * a);  // imaginary part

        (rp, ip)
    }
```

### 2. Factorial Function
  The factorial function `n!` is defined such that for any n > 0, `n!` equals `n * (n-1) * (n-2) *..1`
  ```rust
    pub fn fact(n: u32) -> u32 {
      if n == 1 {
          return n;
      }
  
      n * fact(n - 1)
  }
  ```

### 3. Fib Function
  Returns the nth term of `fib(n)` in the Fibonacci sequence.
  ```rust
    pub fn fib(n: u32) -> u32 {
        let mut cache: HashMap<u32, u32> = HashMap::new();
    
        if n == 0 { return 0 }
        if n == 1 { return 1 }
        
        *cache.entry(n).or_insert_with(|| fib(n - 1) + fib(n - 2))
    }
  ```

### 4. Is Prime
  Checks whether a number `n` is prime. We can take advantage of the fact that prime numbers greater than 3 can be expressed in the form 6k ± 1, where k is a positive integer, avoiding unnecessary checks and improving speed.
  ```rust
    pub fn is_prime(n: u32) -> bool {
      if n <= 1 { return false }
      if n <= 3 { return true }
      if n % 2 == 0 || n % 3 == 0 { return false }
  
      let mut i = 5;
  
      while i*i <= n {
          if n % i == 0 || n % (i + 2) == 0 { return false }
          i += 6
      }
  
      true
  }
  ```
