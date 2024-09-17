use std::fmt::Write;
pub fn classnames(classes: &[(&str, bool)]) -> String {
    let mut result = String::new();
    for &(class, condition) in classes {
        if condition {
            if !result.is_empty() {
                write!(result, " ").unwrap();
            }
            write!(result, "{}", class).unwrap();
        }
    }
    result
}
