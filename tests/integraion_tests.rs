use noorm::greet;

#[test]
fn test_greet() {
    let result = greet("Alice");
    assert_eq!(result, "Hello, Alice!");
}
