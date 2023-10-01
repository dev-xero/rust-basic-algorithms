use rust_basic_algorithms::Quadratic;

#[test]
fn test_from_equation() {
    let equation = String::from("4x2 +2x +8");

    let result = Quadratic::from_equation(&equation);
    let expected = Quadratic {
        equation,
        variables: (4, 2, 8)
    };

    assert_eq!(result, expected, "Quadratic equation should be built from the equation string");
}

#[test]
fn test_from_equation_with_negatives() {
    let equation = String::from("-4x2 -2x +8");

    let result = Quadratic::from_equation(&equation);
    let expected = Quadratic {
        equation,
        variables: (-4, -2, 8)
    };

    assert_eq!(result, expected, "Quadratic equation should be built from the equation string");
}

#[test]
#[should_panic]
fn test_from_equation_with_invalid_eq() {
    let equation = String::from("x2 +2x 8x2");

    Quadratic::from_equation(&equation);
}