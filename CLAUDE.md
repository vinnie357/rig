# Rig Project Development Guide

Rig is a Rust CLI tool for infrastructure management with real-time WebSocket communication to Phoenix channels. This project demonstrates AI-driven development workflow using the Claudio system for comprehensive project analysis and planning.

## Project Overview

### What is Rig?
- **Infrastructure CLI**: Network and application lifecycle management tool
- **Real-time Communication**: WebSocket-based monitoring and control via Phoenix channels  
- **Multi-tenant Architecture**: Hierarchical network/app structure with RFC1035 naming
- **Dual Interface**: Interactive CLI + JSON API for automation workflows

### Current Status
- **Specification Complete**: Comprehensive requirements and architecture defined
- **Implementation Planned**: 5-phase development roadmap with 47 executable tasks
- **Analysis Complete**: Full project discovery, PRD, and planning via Claudio system
- **Ready for Development**: All prerequisites analyzed and documented

## Technology Stack

### Core Dependencies
- **Runtime**: Rust with Tokio async runtime
- **CLI Framework**: clap for argument parsing and command structure
- **HTTP Client**: reqwest for REST API communication
- **WebSocket**: tokio-tungstenite for real-time Phoenix channel integration
- **Serialization**: serde for JSON handling and configuration management
- **Backend**: Phoenix/Elixir channels for real-time communication

### Architecture Patterns
- **Command Pattern**: Modular CLI commands with shared configuration
- **WebSocket Client**: Persistent connection management with automatic reconnection
- **JSON-First**: All operations available via programmatic interface
- **Hierarchical Naming**: RFC1035-compliant network.app naming convention

## Development Workflow

### Getting Started
The project has been fully analyzed using the Claudio system. All development artifacts are available in `.claudio/`:

```bash
# Review project analysis
cat .claudio/discovery.md        # Technology and architecture analysis
cat .claudio/prd.md             # Complete requirements document  
cat .claudio/plan.md            # 5-phase implementation roadmap

# Review executable tasks
ls .claudio/phase*/              # 47 tasks across 5 development phases
cat .claudio/phase1/task-*.md   # Start with Phase 1 tasks
```

### Implementation Phases

#### Phase 1: Project Foundation (8 tasks)
- Rust project initialization with proper structure
- Core CLI framework setup with clap integration
- Configuration management and validation
- Basic error handling and logging infrastructure

#### Phase 2: Core Networking (12 tasks)  
- WebSocket client implementation with connection management
- Phoenix channel protocol integration
- Network discovery and listing capabilities
- Real-time event subscription system

#### Phase 3: Application Management (10 tasks)
- Application lifecycle operations (deploy, start, stop, logs)
- Service configuration and environment management
- Resource monitoring and health checking
- Multi-app coordination within networks

#### Phase 4: Advanced Features (9 tasks)
- Interactive monitoring dashboard
- Bulk operations and automation workflows  
- Plugin system architecture
- Performance optimization and caching

#### Phase 5: Production Readiness (8 tasks)
- Comprehensive testing suite with integration tests
- Documentation and CLI help system
- Error recovery and retry mechanisms
- Packaging and distribution setup

### Quality Assurance

#### Testing Strategy
```bash
# Unit tests for core functionality
cargo test

# Integration tests with Phoenix backend
cargo test --test integration

# CLI interface testing
./test/cli-integration.sh

# WebSocket connection testing  
./test/websocket-integration.sh
```

#### Code Quality
```bash
# Linting and formatting
cargo clippy -- -D warnings
cargo fmt --check

# Security auditing
cargo audit

# Documentation validation
cargo doc --no-deps
```

### Configuration Management

#### Connection Configuration
```toml
[connection]
base_url = "https://api.myplatform.com"
websocket_url = "wss://api.myplatform.com/socket/websocket"
api_key = "your-api-key"
timeout = 30
retry_attempts = 3
```

#### CLI Defaults
```toml
[defaults]
output_format = "table"  # table, json, yaml
log_level = "info"       # error, warn, info, debug
auto_connect = true
follow_logs = false
```

## Key Implementation Notes

### WebSocket Integration
- **Phoenix Channel Protocol**: Implement proper join/leave/push message handling
- **Connection Management**: Automatic reconnection with exponential backoff
- **Message Routing**: Map CLI commands to appropriate channel topics
- **Error Handling**: Graceful degradation when WebSocket unavailable

### CLI Design Principles
- **Intuitive Commands**: `rig network list`, `rig app deploy myapp`
- **Consistent Output**: Structured data with multiple format options
- **Progressive Disclosure**: Basic commands simple, advanced features discoverable
- **Automation Friendly**: All interactive features have non-interactive equivalents

### Error Handling Strategy
- **Network Errors**: Retry with backoff, fallback to REST API when possible
- **Authentication**: Clear error messages with remediation guidance
- **Resource Conflicts**: Detect and suggest resolution strategies
- **Validation Errors**: Helpful error messages with correction suggestions

## Development Commands

### Setup and Build
```bash
# Initialize development environment
cargo build
cargo test

# Run with debug logging
RUST_LOG=debug cargo run -- network list

# Build release version
cargo build --release
```

### Testing and Validation
```bash
# Run all tests
cargo test --all

# Test specific functionality
cargo test websocket
cargo test network_operations
cargo test cli_interface

# Integration testing (requires backend)
cargo test --test integration
```

### Development Tools
```bash
# Generate API documentation
cargo doc --open

# Profile performance
cargo flamegraph --bin rig -- network list

# Check dependencies
cargo tree
cargo outdated
```

## Architecture Decisions

### Why Rust?
- **Performance**: Low-latency WebSocket communication requirements
- **Reliability**: Infrastructure tools need robust error handling
- **Ecosystem**: Excellent async and CLI libraries available
- **Deployment**: Single binary distribution simplifies operations

### Why WebSocket + REST?
- **Real-time Updates**: Infrastructure changes need immediate visibility
- **Fallback Options**: REST API provides reliability when WebSocket unavailable  
- **Automation**: Programmatic access via JSON interface
- **User Experience**: Live updates enhance interactive monitoring

### Why Tokio?
- **Async I/O**: Essential for concurrent WebSocket and HTTP operations
- **Ecosystem**: Broad compatibility with HTTP and WebSocket libraries
- **Performance**: Efficient handling of multiple concurrent connections
- **Maturity**: Production-proven async runtime with excellent tooling

## Integration Patterns

### Phoenix Channel Integration
```rust
// Example WebSocket message handling
match msg.event.as_str() {
    "network_created" => handle_network_event(msg.payload),
    "app_deployed" => handle_app_event(msg.payload),
    "logs" => stream_logs_to_output(msg.payload),
    _ => debug!("Unknown event: {}", msg.event),
}
```

### CLI Command Structure
```rust
// Example command structure
#[derive(Parser)]
enum Commands {
    Network { 
        #[command(subcommand)]
        action: NetworkAction 
    },
    App { 
        #[command(subcommand)]
        action: AppAction 
    },
    Monitor { network: String },
}
```

## Contributing Guidelines

### Code Style
- Follow standard Rust formatting with `cargo fmt`
- Use meaningful variable and function names
- Include docstring comments for public APIs
- Prefer explicit error handling over unwrap()

### Testing Requirements
- Unit tests for all core functionality
- Integration tests for WebSocket communication
- CLI interface testing for user-facing commands
- Error condition testing for failure scenarios

### Documentation Standards
- Update CLI help text for any command changes
- Include usage examples in API documentation
- Document configuration options and their effects
- Maintain this CLAUDE.md file with architectural decisions

## Deployment and Distribution

### Build Targets
```bash
# Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu

# macOS ARM64
cargo build --release --target aarch64-apple-darwin

# Windows x86_64
cargo build --release --target x86_64-pc-windows-msvc
```

### Installation Methods
- **Cargo Install**: `cargo install rig-cli`
- **GitHub Releases**: Pre-built binaries for major platforms
- **Package Managers**: Homebrew, apt, etc. (future)
- **Docker**: Containerized version for CI/CD (future)

---

This project leverages the Claudio system for AI-driven development workflow. All analysis, planning, and task breakdown has been completed and is available in the `.claudio/` directory for reference and execution tracking.