# Project Discovery

Project analysis and technology stack discovery conducted during Claudio installation.

## Project Overview
**Project Name**: (war)rig  
**Project Type**: Command Line Interface (CLI)  
**Primary Language**: Rust  
**Purpose**: CLI client for Max API communication via WebSockets and Phoenix channels

## Technology Stack
- **Language**: Rust
- **Communication**: WebSockets to Phoenix channels
- **Authentication**: API token exchange
- **Architecture**: Client-server architecture with Max API backend
- **Protocol**: RFC1035 compliant naming constraints

## Core Features Identified
- **Authentication**: `rig login` - API token exchange authentication
- **Network Management**: `rig create network` - User network creation with subdomain mapping
- **Application Management**: `rig create app` - Application deployment and management
- **Environment Variables**: `rig create var` - Non-sensitive environment variable management
- **Secret Management**: `rig create secret` - Sensitive data handling as environment variables
- **Deployment**: `rig deploy` - Source code bundling and upload to Max
- **Status Monitoring**: `rig status app/network/dashboard` - Application and network status
- **Logging**: `rig logs app/build/network` - Live log tailing and JSON output
- **Remote Access**: `rig shell` - WebSSH connections to app instances or network control plane
- **Command Execution**: `rig command` - Remote shell command execution
- **Configuration**: `rig details` - App and network configuration details

## Project Structure Analysis
- **Root Directory**: `/Users/vinnie/github/rig/`
- **Documentation**: README.md, docs/ directory with comprehensive guides
- **Subdirectory Projects**: claudio/ (development workflow system)
- **Testing Infrastructure**: Extensive test suite in claudio/test/
- **Example Projects**: examples/ directory with Rust game, web app, and rig examples
- **Research Documentation**: research/ directory with development best practices

## Architecture Patterns
- **CLI Pattern**: Command-based interface with subcommands
- **Network Topology**: app.network.domain.example hostname structure
- **Output Formats**: Interactive (default) and JSON (-o json) for scripting
- **Resource Hierarchy**: Networks contain apps, apps have variables/secrets
- **Real-time Communication**: Live log tailing and WebSocket connections

## Development Environment
- **Build System**: Rust Cargo (inferred from CLI nature)
- **Testing**: Comprehensive test infrastructure with Phoenix LiveView validation app
- **Documentation**: Markdown-based with examples and troubleshooting guides
- **Configuration Management**: TOML-based configuration files (mise.toml in test projects)

## Integration Capabilities
- **Scripting Support**: JSON output format for automation
- **Remote Management**: Shell access and command execution
- **Log Aggregation**: Multi-source log collection (app, build, network)
- **Deployment Automation**: Source bundling and deployment workflows

## Rust CLI Specific Analysis
- **Command Structure**: Hierarchical command system (create, status, logs, etc.)
- **Output Management**: Dual-mode output (interactive vs JSON)
- **Real-time Features**: WebSocket-based live tailing and monitoring
- **Security Model**: Token-based authentication with secret management
- **Network Architecture**: RFC1035 compliant naming with domain mapping

## Next Steps
- Run `/claudio:prd` to create Product Requirements Document
- Run `/claudio:plan` for implementation planning analysis
- Run `/claudio:claudio` for complete workflow execution
- Consider security review for authentication and secret management features
- Analyze Rust-specific development patterns and testing strategies

## Installation Summary
- ✅ Complete .claude/ directory structure created
- ✅ All Claudio commands installed (21+ commands)
- ✅ All agent files installed (64+ agents)
- ✅ Extended context directory structure copied
- ✅ Project discovery completed for Rust CLI project
- ✅ Technology stack identified: Rust + WebSockets + Phoenix channels
- ✅ Core features documented: Authentication, networking, deployment, monitoring
- ✅ Architecture patterns analyzed: CLI, real-time communication, security model