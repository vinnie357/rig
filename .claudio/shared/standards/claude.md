# Rig CLI Project Standards and Conventions

You are working with the Rig CLI project standards and conventions. This context provides shared guidelines, coding standards, and best practices that apply across all phases and tasks of the project.

## Project Overview
The Rig CLI tool is a comprehensive Rust-based command-line interface serving as a client to the Max API platform. This project implements incremental delivery of core functionality through five structured phases over 18 weeks.

## Coding Standards

### Rust Code Standards
- **Edition**: Rust 2021 throughout the project
- **MSRV**: 1.70+ (latest stable recommended)
- **Clippy**: Zero warnings with strict configuration
- **Rustfmt**: 100% compliance with project formatting rules
- **Documentation**: Complete inline documentation for all public APIs

### Code Quality Requirements
- **Test Coverage**: Minimum 90% code coverage across all modules
- **Error Handling**: Comprehensive error handling with user-friendly messages
- **Performance**: CLI responsiveness <2 seconds (P95) for all operations
- **Security**: Zero unsafe code without explicit justification and review

### Architecture Principles
- **Modular Design**: Clear separation of concerns with defined module boundaries
- **Async First**: Full tokio integration for all I/O operations
- **Type Safety**: Strong typing with comprehensive validation
- **Error Recovery**: Graceful degradation and automatic recovery where possible

## Development Practices

### Version Control
- **Git Flow**: Feature branches with pull request reviews
- **Commit Standards**: Conventional commits for automated changelog generation
- **Release Branching**: Separate release branches for stability
- **Tag Management**: Semantic versioning for all releases

### Testing Standards
- **Unit Tests**: Isolated component testing with mocking
- **Integration Tests**: Component interaction testing with controlled environments
- **End-to-End Tests**: Complete workflow validation with Max platform
- **Performance Tests**: Benchmarking and regression detection

### Security Practices
- **Credential Storage**: OS keychain integration with secure fallbacks
- **Network Security**: TLS 1.2+ enforcement for all communications
- **Input Validation**: Comprehensive validation with sanitization
- **Audit Logging**: Complete audit trail for security-sensitive operations

## Communication Standards

### WebSocket and Real-Time Communication
- **Protocol Compliance**: Phoenix channel protocol version compatibility
- **Connection Resilience**: Automatic reconnection with exponential backoff
- **Performance**: <100ms latency for real-time event processing
- **Security**: Proper authentication integration with all connections

### API Integration
- **HTTP Client**: Consistent reqwest usage with retry logic
- **Error Handling**: Comprehensive error types for network, authentication, and API errors
- **Authentication**: Seamless token management and refresh
- **Rate Limiting**: Proper handling of API rate limits and backoff

## User Experience Standards

### CLI Design Principles
- **Intuitive Commands**: Follow established CLI conventions and patterns
- **Consistent Terminology**: Unified language across all commands and documentation
- **Helpful Errors**: Clear error messages with actionable guidance
- **Progressive Disclosure**: Simple commands with advanced options available

### Output Formatting
- **Text Mode**: Human-readable default output with color and formatting
- **JSON Mode**: Structured output for automation and scripting
- **Verbosity Levels**: Appropriate detail levels for different use cases
- **Progress Feedback**: Clear progress indicators for long-running operations

## Performance Standards

### Response Time Requirements
- **CLI Startup**: <100ms for basic commands
- **Authentication**: <3 seconds for login operations
- **Resource Operations**: <2 seconds for CRUD operations
- **Real-Time Events**: <100ms processing latency

### Resource Usage
- **Memory**: <50MB for normal CLI operation
- **CPU**: <10% during normal operation
- **Network**: Efficient connection pooling and reuse
- **Disk**: Minimal local storage with efficient caching

## Security Standards

### Authentication and Authorization
- **Token Management**: Secure storage and automatic refresh
- **Session Handling**: Proper session lifecycle management
- **Access Controls**: Role-based access where applicable
- **Audit Compliance**: Complete logging for security events

### Data Protection
- **Sensitive Data**: No sensitive data in logs or error messages
- **Encryption**: Appropriate encryption for data at rest and in transit
- **Validation**: Input validation and sanitization throughout
- **Error Handling**: No information leakage through error messages

## Documentation Standards

### Code Documentation
- **Public APIs**: Complete rustdoc documentation with examples
- **Internal APIs**: Clear documentation for team development
- **Architecture**: Decision documentation with rationale
- **Configuration**: Complete configuration option documentation

### User Documentation
- **Getting Started**: Clear onboarding with practical examples
- **Command Reference**: Comprehensive command documentation with examples
- **Troubleshooting**: Common issues and resolution guidance
- **Integration**: API documentation for automation and CI/CD

## Quality Assurance

### Code Review Process
- **Peer Review**: All code changes require review and approval
- **Security Review**: Security-sensitive changes require specialist review
- **Performance Review**: Performance-critical changes require benchmarking
- **Documentation Review**: User-facing changes require documentation updates

### Testing Requirements
- **Automated Testing**: Comprehensive test suite in CI/CD pipeline
- **Cross-Platform**: Testing on macOS, Linux, Windows
- **Integration**: Regular testing with Max platform
- **Performance**: Benchmark testing and regression detection

## Integration Guidelines

### Phase Integration
- **Dependencies**: Clear dependency management between phases
- **Handoffs**: Comprehensive documentation for phase transitions
- **Context Sharing**: Shared contexts and resources across phases
- **Communication**: Regular coordination between phase teams

### External Integration
- **Max Platform**: Seamless integration with all Max platform APIs
- **CI/CD Systems**: Automation-friendly interfaces and exit codes
- **Package Managers**: Standard distribution through package managers
- **Monitoring**: Production monitoring and alerting integration

These standards ensure consistent, high-quality implementation across all phases of the Rig CLI project. All team members should reference and follow these guidelines throughout development.