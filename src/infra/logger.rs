use crate::domain::ports::{LogLevel, Logger};

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
