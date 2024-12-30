/// Ensures a Syntax condition is met or panic
///
/// Panics if condition is `false`
///
/// Mainly used for ensuring expected syntax is
/// followed when parsing
pub fn ensure_syntax(message: String, condition: bool) {
    if !condition {
        panic!("Error parsing: {message}");
    }
}
