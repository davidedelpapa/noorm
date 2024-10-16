/// Greets the person with the provided name.
///
/// # Arguments
///
/// * `name` - A string slice that holds the name of the person.
///
/// # Examples
///
/// ```
/// let greeting = noorm::greet("Alice");
/// assert_eq!(greeting, "Hello, Alice!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
