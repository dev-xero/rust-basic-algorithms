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
