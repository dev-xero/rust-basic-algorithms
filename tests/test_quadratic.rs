use rust_basic_algorithms::Quadratic;

#[test]
fn test_from_equation() {
    let equation = String::from("4x2 + 2x + 8");
    let result = Quadratic::from_equation(&equation);
    let expected = Quadratic {
        equation,
        variables: (4, 2, 8)
    };

    assert_eq!(result, expected, "Quadratic equation should be built from the equation string");
}