use rust_basic_algorithms::primes::is_prime;

#[test]
fn test_primes() {
    assert_eq!(true, is_prime(2));
    assert_eq!(true, is_prime(3));
    assert_eq!(true, is_prime(7));
    assert_eq!(false, is_prime(32));
    assert_eq!(true, is_prime(53));
}