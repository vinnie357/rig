# Rig CLI Shared Utilities and Helper Functions

You are working with shared utilities and helper functions for the Rig CLI project. This context provides common patterns, utilities, and reusable components that can be used across all phases and tasks.

## Common Utility Modules

### Error Handling Utilities
```rust
// Standardized error types for consistent error handling
#[derive(Debug, thiserror::Error)]
pub enum RigError {
    #[error("Authentication failed: {message}")]
    Authentication { message: String },
    
    #[error("Network error: {source}")]
    Network { source: reqwest::Error },
    
    #[error("Configuration error: {message}")]
    Configuration { message: String },
    
    #[error("Validation error: {field} - {message}")]
    Validation { field: String, message: String },
    
    #[error("Resource not found: {resource_type} '{name}'")]
    ResourceNotFound { resource_type: String, name: String },
}

// Result type alias for consistent error handling
pub type RigResult<T> = Result<T, RigError>;

// Error conversion utilities
impl From<reqwest::Error> for RigError {
    fn from(err: reqwest::Error) -> Self {
        RigError::Network { source: err }
    }
}
```

### Configuration Utilities
```rust
// Configuration loading with hierarchy support
pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load_with_hierarchy() -> RigResult<Config> {
        // Load configuration from multiple sources with precedence
        let mut config = Config::default();
        
        // 1. Built-in defaults (lowest priority)
        // 2. System config
        // 3. User config
        // 4. Project config
        // 5. Environment variables
        // 6. CLI flags (highest priority)
        
        Ok(config)
    }
    
    pub fn validate_config(config: &Config) -> RigResult<()> {
        // Comprehensive configuration validation
        Ok(())
    }
}
```

### HTTP Client Utilities
```rust
// Shared HTTP client with consistent configuration
pub struct HttpClientBuilder;

impl HttpClientBuilder {
    pub fn new() -> Self {
        Self
    }
    
    pub fn with_auth(mut self, auth: &AuthContext) -> Self {
        // Add authentication headers
        self
    }
    
    pub fn with_retry(mut self, config: RetryConfig) -> Self {
        // Add retry logic with exponential backoff
        self
    }
    
    pub fn build(self) -> RigResult<reqwest::Client> {
        // Build configured HTTP client
        Ok(reqwest::Client::new())
    }
}

// Retry logic utilities
pub async fn retry_with_backoff<F, T, E>(
    operation: F,
    config: RetryConfig,
) -> Result<T, E>
where
    F: Fn() -> Result<T, E>,
{
    // Implement exponential backoff with jitter
    todo!()
}
```

### Validation Utilities
```rust
// RFC1035 DNS name validation
pub fn validate_dns_name(name: &str) -> RigResult<()> {
    // Implement RFC1035 compliant validation
    if name.is_empty() || name.len() > 253 {
        return Err(RigError::Validation {
            field: "name".to_string(),
            message: "Name must be 1-253 characters".to_string(),
        });
    }
    
    // Additional RFC1035 validation rules
    Ok(())
}

// Resource name uniqueness validation
pub async fn validate_uniqueness(
    resource_type: &str,
    name: &str,
    context: &ApiContext,
) -> RigResult<()> {
    // Check uniqueness against Max platform
    Ok(())
}

// Input sanitization utilities
pub fn sanitize_input(input: &str) -> String {
    // Remove potentially dangerous characters
    input.trim().to_string()
}
```

### Output Formatting Utilities
```rust
// Consistent output formatting across commands
pub enum OutputFormat {
    Text,
    Json,
    Table,
}

pub struct OutputFormatter {
    format: OutputFormat,
    color: bool,
}

impl OutputFormatter {
    pub fn new(format: OutputFormat, color: bool) -> Self {
        Self { format, color }
    }
    
    pub fn format_resource<T: Serialize>(&self, resource: &T) -> RigResult<String> {
        match self.format {
            OutputFormat::Json => serde_json::to_string_pretty(resource)
                .map_err(|e| RigError::Configuration {
                    message: format!("JSON serialization failed: {}", e),
                }),
            OutputFormat::Text => self.format_text(resource),
            OutputFormat::Table => self.format_table(resource),
        }
    }
    
    fn format_text<T: Serialize>(&self, resource: &T) -> RigResult<String> {
        // Human-readable text formatting
        Ok(String::new())
    }
    
    fn format_table<T: Serialize>(&self, resource: &T) -> RigResult<String> {
        // Table formatting for lists
        Ok(String::new())
    }
}
```

### Progress Tracking Utilities
```rust
// Progress reporting for long-running operations
pub struct ProgressReporter {
    total: u64,
    current: u64,
    start_time: Instant,
}

impl ProgressReporter {
    pub fn new(total: u64) -> Self {
        Self {
            total,
            current: 0,
            start_time: Instant::now(),
        }
    }
    
    pub fn update(&mut self, current: u64) {
        self.current = current;
        self.display_progress();
    }
    
    fn display_progress(&self) {
        let percentage = (self.current as f64 / self.total as f64) * 100.0;
        let elapsed = self.start_time.elapsed();
        let eta = if self.current > 0 {
            elapsed * (self.total - self.current) / self.current
        } else {
            Duration::from_secs(0)
        };
        
        println!("Progress: {:.1}% ({}/{}) ETA: {:?}", 
                percentage, self.current, self.total, eta);
    }
}
```

### Logging Utilities
```rust
// Consistent logging configuration
pub fn setup_logging(verbosity: u8) -> RigResult<()> {
    let level = match verbosity {
        0 => tracing::Level::WARN,
        1 => tracing::Level::INFO,
        2 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };
    
    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .init();
    
    Ok(())
}

// Structured logging macros
#[macro_export]
macro_rules! log_operation {
    ($level:expr, $operation:expr, $($key:expr => $value:expr),*) => {
        match $level {
            LogLevel::Info => tracing::info!(
                operation = $operation,
                $($key = $value),*
            ),
            LogLevel::Debug => tracing::debug!(
                operation = $operation,
                $($key = $value),*
            ),
            _ => {}
        }
    };
}
```

### Testing Utilities
```rust
// Test environment setup
pub struct TestEnvironment {
    temp_dir: TempDir,
    mock_server: MockServer,
    config: Config,
}

impl TestEnvironment {
    pub async fn new() -> RigResult<Self> {
        let temp_dir = TempDir::new()?;
        let mock_server = MockServer::start().await;
        let config = Config::test_default();
        
        Ok(Self {
            temp_dir,
            mock_server,
            config,
        })
    }
    
    pub fn config_path(&self) -> PathBuf {
        self.temp_dir.path().join("config.toml")
    }
    
    pub fn mock_endpoint(&self, path: &str) -> String {
        format!("{}{}", self.mock_server.uri(), path)
    }
}

// Mock server utilities
pub fn setup_auth_mock(server: &MockServer) {
    Mock::given(method("POST"))
        .and(path("/auth/login"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({
                "access_token": "test_token",
                "expires_in": 3600
            })))
        .mount(server);
}
```

### File System Utilities
```rust
// File bundling and compression
pub struct FileBundler;

impl FileBundler {
    pub async fn bundle_directory(
        path: &Path,
        exclude_patterns: &[String],
    ) -> RigResult<Vec<u8>> {
        // Create compressed archive of directory
        // Exclude files matching patterns
        // Return compressed bytes
        Ok(Vec::new())
    }
    
    pub fn calculate_bundle_size(path: &Path) -> RigResult<u64> {
        // Calculate total size of files to be bundled
        Ok(0)
    }
    
    pub async fn upload_with_progress<F>(
        data: Vec<u8>,
        endpoint: &str,
        progress_callback: F,
    ) -> RigResult<()>
    where
        F: Fn(u64, u64),
    {
        // Upload with progress reporting
        Ok(())
    }
}
```

### Time and Duration Utilities
```rust
// Human-readable duration formatting
pub fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    }
}

// Timeout utilities
pub async fn with_timeout<T>(
    duration: Duration,
    future: impl Future<Output = T>,
) -> RigResult<T> {
    tokio::time::timeout(duration, future)
        .await
        .map_err(|_| RigError::Configuration {
            message: "Operation timed out".to_string(),
        })
}
```

## Usage Guidelines

### Import Patterns
```rust
// Standard imports for all modules
use crate::shared::{
    RigError, RigResult,
    OutputFormatter, OutputFormat,
    ProgressReporter,
    ValidationError,
};
```

### Error Handling Pattern
```rust
// Consistent error handling across the codebase
pub async fn example_operation() -> RigResult<String> {
    let client = HttpClientBuilder::new()
        .with_auth(&auth_context)
        .with_retry(RetryConfig::default())
        .build()?;
    
    let response = client.get("https://api.example.com/data")
        .send()
        .await?;
    
    response.text().await.map_err(Into::into)
}
```

### Configuration Pattern
```rust
// Standard configuration loading
pub fn load_config() -> RigResult<Config> {
    ConfigLoader::load_with_hierarchy()
        .and_then(|config| {
            ConfigLoader::validate_config(&config)?;
            Ok(config)
        })
}
```

These utilities provide consistent patterns and functionality across all phases of the Rig CLI project. Use these shared components to maintain consistency and reduce code duplication throughout the implementation.