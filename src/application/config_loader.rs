// Application Layer - Configuration Loader Implementation
//
// Environment-based configuration loader.
// According to rust.guidelines.md, this should be in Application layer
// as it involves parsing environment data (not pure I/O like file system).

use crate::domain::config::Md2TomlConfig;
use crate::domain::errors::Result;
use crate::domain::ports::ConfigLoader;
use std::path::Path;

/// Environment-based configuration loader
///
/// Loads configuration from environment variables and provides
/// sensible defaults.
pub struct EnvConfigLoader;

impl EnvConfigLoader {
    pub fn new() -> Self {
        Self
    }
}

impl Default for EnvConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigLoader for EnvConfigLoader {
    fn load_config(&self, _path: &Path) -> Result<Md2TomlConfig> {
        // For now, we load from environment
        // Future: implement loading from TOML file
        self.load_from_env()
    }

    fn load_from_env(&self) -> Result<Md2TomlConfig> {
        let mut config = Md2TomlConfig::default();

        // Load verbose flag from environment
        if std::env::var("MD2TOML_VERBOSE").unwrap_or_default() == "true" {
            config.verbose = true;
        }

        // Load skip_hidden flag
        if let Ok(val) = std::env::var("MD2TOML_SKIP_HIDDEN") {
            config.skip_hidden = val != "false";
        }

        // Load max_file_size
        if let Ok(val) = std::env::var("MD2TOML_MAX_FILE_SIZE") {
            if let Ok(size) = val.parse::<u64>() {
                config.max_file_size = Some(size);
            }
        }

        // Load overwrite flag
        if std::env::var("MD2TOML_OVERWRITE").unwrap_or_default() == "true" {
            config.overwrite = true;
        }

        // Load parallel flag
        if std::env::var("MD2TOML_PARALLEL").unwrap_or_default() == "true" {
            config.parallel = true;

            // Load worker threads if parallel is enabled
            if let Ok(val) = std::env::var("MD2TOML_WORKER_THREADS") {
                if let Ok(threads) = val.parse::<usize>() {
                    config.worker_threads = Some(threads);
                }
            }
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_from_env_default() {
        // Clear all environment variables to ensure clean test
        std::env::remove_var("MD2TOML_VERBOSE");
        std::env::remove_var("MD2TOML_SKIP_HIDDEN");
        std::env::remove_var("MD2TOML_OVERWRITE");
        std::env::remove_var("MD2TOML_PARALLEL");
        std::env::remove_var("MD2TOML_WORKER_THREADS");
        std::env::remove_var("MD2TOML_MAX_FILE_SIZE");

        let loader = EnvConfigLoader::new();
        let config = loader.load_from_env().unwrap();

        assert!(!config.verbose);
        assert!(config.skip_hidden);
        assert!(!config.overwrite);
        assert!(!config.parallel);
        assert_eq!(config.worker_threads, None);
        assert_eq!(config.max_file_size, None);
    }

    #[test]
    fn test_load_from_env_verbose() {
        // Clear all env vars first
        std::env::remove_var("MD2TOML_VERBOSE");
        std::env::remove_var("MD2TOML_SKIP_HIDDEN");
        std::env::remove_var("MD2TOML_OVERWRITE");
        std::env::remove_var("MD2TOML_PARALLEL");
        std::env::remove_var("MD2TOML_WORKER_THREADS");
        std::env::remove_var("MD2TOML_MAX_FILE_SIZE");

        // Set the one we want to test
        std::env::set_var("MD2TOML_VERBOSE", "true");

        let loader = EnvConfigLoader::new();
        let config = loader.load_from_env().unwrap();

        assert!(config.verbose);

        // Clean up
        std::env::remove_var("MD2TOML_VERBOSE");
    }

    #[test]
    fn test_load_from_env_max_file_size() {
        // Clear all env vars first
        std::env::remove_var("MD2TOML_VERBOSE");
        std::env::remove_var("MD2TOML_SKIP_HIDDEN");
        std::env::remove_var("MD2TOML_OVERWRITE");
        std::env::remove_var("MD2TOML_PARALLEL");
        std::env::remove_var("MD2TOML_WORKER_THREADS");
        std::env::remove_var("MD2TOML_MAX_FILE_SIZE");

        // Set the one we want to test
        std::env::set_var("MD2TOML_MAX_FILE_SIZE", "1024");

        let loader = EnvConfigLoader::new();
        let config = loader.load_from_env().unwrap();

        assert_eq!(config.max_file_size, Some(1024));

        // Clean up
        std::env::remove_var("MD2TOML_MAX_FILE_SIZE");
    }

    #[test]
    fn test_load_from_env_parallel() {
        // Clear all env vars first
        std::env::remove_var("MD2TOML_VERBOSE");
        std::env::remove_var("MD2TOML_SKIP_HIDDEN");
        std::env::remove_var("MD2TOML_OVERWRITE");
        std::env::remove_var("MD2TOML_PARALLEL");
        std::env::remove_var("MD2TOML_WORKER_THREADS");
        std::env::remove_var("MD2TOML_MAX_FILE_SIZE");

        // Set the ones we want to test
        std::env::set_var("MD2TOML_PARALLEL", "true");
        std::env::set_var("MD2TOML_WORKER_THREADS", "4");

        let loader = EnvConfigLoader::new();
        let config = loader.load_from_env().unwrap();

        assert!(config.parallel);
        assert_eq!(config.worker_threads, Some(4));

        // Clean up
        std::env::remove_var("MD2TOML_PARALLEL");
        std::env::remove_var("MD2TOML_WORKER_THREADS");
    }
}
