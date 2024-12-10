/// Utility macro used internally to allow optional parameters
/// in macros.
///
/// This macro is not intended to be used directly.
#[macro_export]
macro_rules! some_or_none {
    () => {
        None
    };
    ($entity:expr) => {
        Some($entity)
    };
}
