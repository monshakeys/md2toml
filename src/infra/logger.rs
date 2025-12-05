// Infrastructure Layer - Logger Implementation
//
// Simple logger that outputs to stdout/stderr.
// This follows the Rust guidelines: Infrastructure must use Logger Port,
// never directly use println! in infrastructure code.

use crate::domain::ports::{LogLevel, Logger};

/// A simple logger adapter that uses println!/eprintln! directly.
///
/// This implementation:
/// - Satisfies the Port pattern (implements Logger trait from Domain)
/// - Keeps Infrastructure layer free from direct println! usage
/// - Avoids dependency on the `log` crate (simpler, less dependencies)
/// - Suitable for CLI tools that don't need complex logging infrastructure
pub struct SimpleLogger;

impl Logger for SimpleLogger {
    fn info(&self, message: &str) {
        println!("{}", message);
    }

    fn warn(&self, message: &str) {
        eprintln!("WARNING: {}", message);
    }

    fn error(&self, message: &str) {
        eprintln!("ERROR: {}", message);
    }

    fn debug(&self, _message: &str) {
        // Only output debug messages in debug builds
        #[cfg(debug_assertions)]
        {
            println!("DEBUG: {}", _message);
        }
    }

    fn log(&self, level: LogLevel, message: &str, _context: Option<&str>) {
        let full_message = message.to_string();
        match level {
            LogLevel::Debug => self.debug(&full_message),
            LogLevel::Info => self.info(&full_message),
            LogLevel::Warning => self.warn(&full_message),
            LogLevel::Error => self.error(&full_message),
        }
    }

    fn is_debug_enabled(&self) -> bool {
        #[cfg(debug_assertions)]
        {
            true
        }
        #[cfg(not(debug_assertions))]
        {
            false
        }
    }
}
