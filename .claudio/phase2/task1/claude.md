# Task 2.1: WebSocket Foundation Agent

You are a specialized agent focused on Task 2.1: WebSocket Foundation. Your expertise is in implementing robust WebSocket clients using Rust, connection lifecycle management, message serialization, and real-time communication patterns.

## Task Overview
- **Description**: Implement WebSocket client using tokio-tungstenite with connection lifecycle management, message serialization, and heartbeat mechanisms
- **Priority**: High
- **Estimated Effort**: 1 week
- **Dependencies**: Phase 1 (Foundation) completion
- **Timeline**: Week 5 of Phase 2

## Acceptance Criteria
- [ ] WebSocket client implemented using tokio-tungstenite with robust connection handling
- [ ] Connection lifecycle management covers connect, disconnect, and comprehensive error handling
- [ ] WebSocket message serialization/deserialization supports JSON and binary formats
- [ ] Ping/pong heartbeat mechanism maintains connection health monitoring
- [ ] Connection state tracking provides real-time status and diagnostic information
- [ ] Performance meets latency requirements (<100ms message processing)

## Technical Specifications

**WebSocket Client Architecture**:
```rust
pub struct WebSocketClient {
    stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    config: WebSocketConfig,
    state: ConnectionState,
    heartbeat_handle: Option<JoinHandle<()>>,
}

pub struct WebSocketConfig {
    url: String,
    heartbeat_interval: Duration,
    connection_timeout: Duration,
    message_queue_size: usize,
}

#[derive(Debug, Clone)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Error(String),
}
```

**Message Framework**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    id: Option<String>,
    payload: MessagePayload,
    timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagePayload {
    Text(String),
    Binary(Vec<u8>),
    Json(serde_json::Value),
    Ping,
    Pong,
}
```

## Implementation Guidelines

**Connection Management**:
1. **Async Connection**: Use tokio-tungstenite for full async WebSocket support
2. **TLS Support**: Native TLS for secure connections to Max platform
3. **Connection Pooling**: Reuse connections for efficiency and performance
4. **Graceful Shutdown**: Proper connection cleanup and resource management

**Message Handling**:
1. **Serialization**: JSON primary format with binary fallback support
2. **Type Safety**: Strong typing for all message formats and payloads
3. **Error Recovery**: Graceful handling of malformed or unexpected messages
4. **Performance**: Zero-copy deserialization where possible

**Heartbeat System**:
1. **Ping/Pong**: Regular heartbeat to maintain connection health
2. **Timeout Detection**: Automatic detection of connection issues
3. **Health Monitoring**: Real-time connection quality metrics
4. **Adaptive Timing**: Dynamic heartbeat intervals based on connection quality

## Quality Requirements

**Performance Standards**:
- **Message Processing**: <100ms latency for all message types
- **Connection Time**: <500ms for initial WebSocket establishment
- **Memory Usage**: <50MB for WebSocket client overhead
- **CPU Usage**: <5% during normal operation

**Reliability Standards**:
- **Connection Stability**: Handle network interruptions gracefully
- **Message Delivery**: Guarantee delivery order and completeness
- **Error Recovery**: Automatic recovery from transient failures
- **Resource Management**: Proper cleanup and leak prevention

**Security Standards**:
- **TLS Enforcement**: Require TLS 1.2+ for all connections
- **Certificate Validation**: Proper SSL certificate verification
- **Authentication Integration**: Seamless auth token integration
- **Data Protection**: Secure handling of sensitive message content

## Implementation Structure

**WebSocket Module** (`core/src/websocket/`):
```
websocket/
├── mod.rs              // Public WebSocket interface
├── client.rs           // Core WebSocket client implementation
├── connection.rs       // Connection lifecycle management
├── heartbeat.rs        // Ping/pong heartbeat system
├── messages.rs         // Message serialization and types
├── state.rs            // Connection state management
└── errors.rs           // WebSocket-specific error types
```

**Message Types** (`core/src/websocket/messages.rs`):
```rust
// Core message types for WebSocket communication
pub trait WebSocketMessage: Serialize + DeserializeOwned {
    fn message_type(&self) -> &'static str;
    fn validate(&self) -> Result<(), ValidationError>;
}

// Message serialization utilities
pub struct MessageSerializer;
impl MessageSerializer {
    pub fn serialize<T: WebSocketMessage>(msg: &T) -> Result<String>;
    pub fn deserialize<T: WebSocketMessage>(data: &str) -> Result<T>;
}
```

## Testing Requirements

**Unit Testing**:
- Connection lifecycle state transitions
- Message serialization/deserialization accuracy
- Heartbeat timing and failure detection
- Error handling for various failure scenarios

**Integration Testing**:
- End-to-end WebSocket communication with test server
- TLS connection validation and certificate handling
- Authentication token integration and renewal
- Performance testing under various load conditions

**Mock Infrastructure**:
- WebSocket test server for isolated testing
- Network condition simulation (latency, packet loss)
- Authentication server mock for token validation
- Performance benchmark utilities

## Deliverables

**Core WebSocket Client**:
- Complete tokio-tungstenite WebSocket client implementation
- Connection lifecycle management with state tracking
- Comprehensive error handling and recovery mechanisms
- Performance optimization for real-time communication

**Message Framework**:
- Type-safe message serialization/deserialization
- JSON and binary message format support
- Message validation and error recovery
- Zero-copy optimization where possible

**Heartbeat System**:
- Ping/pong heartbeat implementation with configurable intervals
- Connection health monitoring and diagnostic reporting
- Automatic failure detection and recovery initiation
- Adaptive heartbeat timing based on connection quality

**Testing Infrastructure**:
- Comprehensive unit test suite for all WebSocket components
- Integration testing framework with mock server
- Performance benchmarking and validation utilities
- Network condition simulation and testing tools

## Context Integration
- **Phase Context**: Reference `../claude.md` for phase coordination and communication requirements
- **Previous Phase**: Build upon Phase 1 authentication system and configuration management
- **Next Task**: Prepare WebSocket foundation for Task 2.2 Phoenix channel protocol integration
- **Project Context**: Align with Max platform WebSocket endpoints and requirements

## Success Validation

**Functional Validation**:
- WebSocket client connects successfully to Max platform endpoints
- Message serialization handles all expected data types correctly
- Heartbeat system maintains connection health monitoring
- Connection state tracking provides accurate real-time status

**Performance Validation**:
- Message processing latency consistently <100ms
- Connection establishment time <500ms
- Memory usage remains <50MB for WebSocket operations
- CPU usage <5% during normal message processing

**Integration Validation**:
- Authentication tokens integrate seamlessly with WebSocket headers
- Configuration system provides proper WebSocket settings
- Error handling integrates with CLI error reporting system
- Logging provides appropriate detail for debugging and monitoring

## Performance Requirements
- **Message Latency**: <100ms processing time (P95)
- **Connection Time**: <500ms establishment (P95)
- **Throughput**: >1000 messages/second sustained
- **Memory**: <50MB overhead for WebSocket client

## Security Requirements
- **TLS**: Enforce TLS 1.2+ for all WebSocket connections
- **Authentication**: Integrate Phase 1 authentication tokens
- **Certificate Validation**: Proper SSL certificate verification
- **Data Protection**: Secure handling of message content

## Error Handling Scenarios

**Connection Errors**:
- Network timeout and connection failure recovery
- TLS handshake errors and certificate issues
- DNS resolution failures and endpoint availability
- Authentication failures and token renewal

**Message Errors**:
- Malformed message handling and recovery
- Serialization/deserialization error management
- Unknown message type handling
- Message size limits and validation

**Protocol Errors**:
- WebSocket protocol violation handling
- Heartbeat timeout and connection health issues
- Connection state inconsistency recovery
- Resource exhaustion and backpressure management

## Next Steps After Completion
1. Update task status with implementation details and performance metrics
2. Validate WebSocket client with Max platform test environments
3. Prepare WebSocket foundation for Task 2.2 Phoenix channel integration
4. Document WebSocket client patterns and integration guidelines

This WebSocket foundation provides the critical communication layer that enables all real-time features of the Rig CLI tool. Focus on creating a robust, performant, and reliable WebSocket client that will support the Phoenix channel protocol and all subsequent real-time communication requirements.