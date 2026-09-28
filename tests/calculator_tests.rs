use bionic_agent_tools::tools::calculator::{Calculator, CalculatorError};

#[test]
fn test_evaluate_simple_addition() {
    // 1. ARRANGE
    let calc = Calculator::new();
    let expr = "12 + 28";

    // 2. ACT
    let result = calc.evaluate(expr);

    // 3. ASSERT
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 40.0);
}

#[test]
fn test_evaluate_operator_precedence() {
    // 1. ARRANGE
    let calc = Calculator::new();
    let expr = "2 + 3 * 4";

    // 2. ACT
    let result = calc.evaluate(expr);

    // 3. ASSERT
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 14.0);
}

#[test]
fn test_evaluate_parentheses_and_floating_point() {
    // 1. ARRANGE
    let calc = Calculator::new();
    let expr = "(10.5 - 2.5) * 2 / 4";

    // 2. ACT
    let result = calc.evaluate(expr);

    // 3. ASSERT
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 4.0);
}

#[test]
fn test_evaluate_functions_sqrt_and_abs() {
    // 1. ARRANGE
    let calc = Calculator::new();
    let expr = "sqrt(16) + abs(-10)";

    // 2. ACT
    let result = calc.evaluate(expr);

    // 3. ASSERT
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 14.0);
}

#[test]
fn test_evaluate_division_by_zero_returns_error() {
    // 1. ARRANGE
    let calc = Calculator::new();
    let expr = "42 / 0";

    // 2. ACT
    let result = calc.evaluate(expr);

    // 3. ASSERT
    assert!(matches!(result, Err(CalculatorError::DivisionByZero)));
}

#[test]
fn test_evaluate_mismatched_parentheses_returns_error() {
    // 1. ARRANGE
    let calc = Calculator::new();
    let expr = "(10 + 5";

    // 2. ACT
    let result = calc.evaluate(expr);

    // 3. ASSERT
    assert!(matches!(result, Err(CalculatorError::InvalidExpression(_))));
}

#[test]
fn test_tool_execute_json_output() {
    // 1. ARRANGE
    let calc = Calculator::new();
    let payload = serde_json::json!({
        "expression": "sqrt(144) * 2"
    });

    // 2. ACT
    let response = calc.execute_tool(&payload).expect("should execute tool");

    // 3. ASSERT
    assert_eq!(response["status"], "success");
    assert_eq!(response["result"], 24.0);
    assert_eq!(response["expression"], "sqrt(144) * 2");
}
