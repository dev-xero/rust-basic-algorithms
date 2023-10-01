use rust_basic_algorithms::fact::fact;

#[test]
fn test_fact_six() {
    let result = fact(6);
    let expected = 720;

    assert_eq!(result, expected, "Should compute the factorial of six");
}
