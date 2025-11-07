/// Debug printing macro that only prints when DEBUG environment variable is set
///
/// Usage:
/// ```
/// use hegel_pm::debug;
/// let count = 5;
/// debug!("Loading {} projects", count);
/// debug!("Cache hit");
/// ```
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if std::env::var("DEBUG").is_ok() {
            println!($($arg)*);
        }
    };
}

#[cfg(test)]
mod tests {
    use std::env;

    #[test]
    fn test_debug_macro_with_env() {
        env::set_var("DEBUG", "1");
        // This should print when DEBUG is set
        debug!("Test message");
        env::remove_var("DEBUG");
    }

    #[test]
    fn test_debug_macro_without_env() {
        env::remove_var("DEBUG");
        // This should not print when DEBUG is not set
        debug!("This should not print");
    }

    #[test]
    fn test_debug_macro_formatting() {
        env::set_var("DEBUG", "1");
        let count = 42;
        debug!("Count is {}", count);
        debug!("Multiple {} values: {}", "formatted", 123);
        env::remove_var("DEBUG");
    }
}
