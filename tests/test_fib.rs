use rust_basic_algorithms::fib::fib;

#[test]
fn test_fib() {
    let result = fib(10);
    let expected = 55;

    assert_eq!(result, expected, "Should return the 10th term in the fibonacci sequence");
}