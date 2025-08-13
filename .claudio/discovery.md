# Project Discovery: Rig CLI Tool

**Project**: Rig CLI  
**Location**: `/Users/vinnie/github/rig`  
**Analysis Date**: 2025-08-12  
**Status**: Specification-only project (no implementation)

## Executive Summary

Rig is a planned Rust command-line interface that will serve as a client to the Max API platform. Currently exists as a specification document only (readme.md) with no implementation code present.

## Project Structure

```
/Users/vinnie/github/rig/
├── readme.md          # Complete CLI specification
├── claudio/           # Claudio system (ignore for analysis)
└── [missing]          # No Cargo.toml or source code yet
```

## Technology Stack Analysis

### Required Technologies
- **Language**: Rust
- **Async Runtime**: Tokio (for WebSocket communication)
- **HTTP Client**: reqwest or similar for authentication
- **WebSocket**: tokio-tungstenite for Phoenix channel communication
- **CLI Framework**: clap for command parsing
- **Serialization**: serde for JSON handling

### Communication Architecture
- **Authentication**: HTTP token exchange with Max API
- **Primary Communication**: WebSocket upgrade to Phoenix channels
- **Protocol**: Structured messaging over WebSocket

## Feature Analysis

### Core Commands Specified

1. **Authentication**
   - `rig login` - Token exchange authentication

2. **Resource Management**
   - `rig create network` - Create user networks (subdomain.domain.example)
   - `rig create app` - Create applications (app.network.domain.example)
   - `rig create var` - Environment variable management
   - `rig create secret` - Sensitive environment variable management

3. **Deployment**
   - `rig deploy` - Bundle and upload source code archives

4. **Monitoring**
   - `rig status app|network|dashboard` - Status information
   - `rig logs app|build|network` - Log streaming with live tail

5. **Remote Access**
   - `rig shell` - WebSSH connections to app instances or network control plane
   - `rig command` - Execute remote shell commands

6. **Configuration**
   - `rig details` - Get app/network configuration details

### Output Modes
- **Interactive** (default): User-friendly output
- **JSON** (`-o json`): Scriptable structured output

## Domain Model

### Naming Constraints
- RFC1035 compliant naming
- Structure: `app.network.domain.example`
- App names unique per network
- Network names globally unique

## Development Requirements

### Missing Implementation
- [ ] Cargo.toml with dependencies
- [ ] Source code structure
- [ ] CLI argument parsing
- [ ] Authentication module
- [ ] WebSocket client
- [ ] Phoenix channel protocol
- [ ] Command implementations
- [ ] Error handling
- [ ] Testing framework

### Recommended Project Structure
```
src/
├── main.rs              # CLI entry point
├── cli/                 # Command definitions
├── auth/                # Authentication module
├── client/              # Max API client
├── channels/            # Phoenix channel communication
├── commands/            # Command implementations
└── utils/               # Shared utilities
```

## Implementation Roadmap

### Phase 1: Foundation (Weeks 1-4)
- Set up Cargo project with dependencies
- Implement basic CLI structure with clap
- Create authentication module
- Basic HTTP client for token exchange

### Phase 2: Core Communication (Weeks 5-8)
- WebSocket client implementation
- Phoenix channel protocol support
- Connection management and error handling

### Phase 3: Command Implementation (Weeks 9-12)
- Resource management commands (network, app, var, secret)
- Status and monitoring commands
- Deploy functionality

### Phase 4: Advanced Features (Weeks 13-16)
- Shell and remote command execution
- Log streaming with live tail
- JSON output mode
- Comprehensive testing

## Risk Assessment

### Technical Challenges
- **WebSocket Stability**: Maintaining persistent connections
- **Phoenix Protocol**: Correctly implementing channel communication
- **Error Handling**: Graceful degradation and user feedback
- **Authentication**: Secure token management

### Mitigation Strategies
- Use established Rust WebSocket libraries
- Study Phoenix channel protocol documentation
- Implement comprehensive error types
- Follow Rust security best practices

## Next Steps

1. **Create PRD**: Define detailed requirements and user stories
2. **Implementation Plan**: Detailed task breakdown with timelines
3. **Project Setup**: Initialize Cargo project with dependencies
4. **Architecture Design**: Define module structure and interfaces

## Quality Metrics

- **Specification Completeness**: 90% - Well-defined commands and behavior
- **Technical Feasibility**: 95% - Standard Rust ecosystem capabilities
- **Implementation Readiness**: 20% - Missing all code infrastructure

This discovery provides the foundation for implementing the rig CLI tool based on the comprehensive specification provided.