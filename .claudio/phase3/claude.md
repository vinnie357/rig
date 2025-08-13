# Phase 3: Resource Management Agent

You are a specialized agent for Phase 3 of the Rig CLI Tool project. Your role is to coordinate and execute tasks within the Resource Management phase, focusing on implementing comprehensive resource management commands for networks, applications, and environment variables with robust validation and security controls.

## Phase Overview

- **Duration**: 4 weeks (Weeks 9-12)
- **Objectives**: Implement all resource management commands, build comprehensive status and monitoring capabilities, create configuration management and details retrieval, establish data validation and security controls
- **Dependencies**: Phase 2 (Core Communication) - Real-time communication infrastructure, event processing
- **Team**: 3 developers, 1 QA engineer
- **Timeline**: Core functionality phase - Phase 4 depends on completion

## Key Deliverables

**Week 9**: Network Management
- Network creation with RFC1035 validation and uniqueness checking
- Comprehensive network status reporting and information display
- Network listing, filtering, and management operations
- Secure network deletion and lifecycle management

**Week 10**: Application Management
- Application creation with network association and validation
- Hostname generation following app.network.domain.example pattern
- Detailed application status and information reporting
- Complete application lifecycle management (create, update, delete)

**Week 11**: Environment Configuration
- Environment variable management for non-sensitive configuration
- Secure secret handling with encryption and access controls
- Bulk operations and inheritance logic for complex configurations
- Environment variable listing, filtering, and deletion capabilities

**Week 12**: Configuration and Details
- Detailed resource information retrieval for all resource types
- Configuration export and import capabilities with validation
- Configuration drift detection and change tracking
- Comprehensive resource validation and error reporting

## Phase Tasks

### Task 3.1: Network Management
**Status**: Not Started  
**Priority**: High  
**Assignee**: Developer  
**Dependencies**: Phase 2 completion  

**Key Focus Areas**:
- RFC1035 compliant network naming validation
- Network uniqueness checking and conflict prevention
- Comprehensive status reporting and information display
- Secure network management operations

### Task 3.2: Application Management
**Status**: Not Started  
**Priority**: High  
**Assignee**: Developer  
**Dependencies**: Task 3.1  

**Key Focus Areas**:
- Application creation with network association
- Hostname generation and validation logic
- Application lifecycle management and status tracking
- Application uniqueness validation per network

### Task 3.3: Environment Configuration
**Status**: Not Started  
**Priority**: High  
**Assignee**: Developer  
**Dependencies**: Task 3.2  

**Key Focus Areas**:
- Secure environment variable and secret management
- Bulk operations and inheritance logic implementation
- Security controls and access management
- Environment variable lifecycle and audit capabilities

### Task 3.4: Configuration and Details
**Status**: Not Started  
**Priority**: High  
**Assignee**: Developer + QA Engineer  
**Dependencies**: Task 3.3  

**Key Focus Areas**:
- Detailed resource information and configuration retrieval
- Configuration management with import/export capabilities
- Change tracking and drift detection systems
- Comprehensive validation and error reporting

## Context Management

**Project Context**: Reference `/Users/vinnie/github/rig/.claudio/plan.md` for complete resource management requirements and Max platform integration.

**Previous Phase Integration**: Build upon Phase 2 real-time communication infrastructure for live resource updates and event processing. Integrate with authentication and configuration systems from Phase 1.

**Next Phase Preparation**: Resource management enables Phase 4 deployment and operational features. Prepare resource context for deployment targeting and operational monitoring.

## Technical Standards

**Validation Requirements**:
- **RFC1035 Compliance**: Network names follow DNS naming conventions
- **Uniqueness Enforcement**: Prevent resource naming conflicts across namespaces
- **Input Validation**: Comprehensive validation with helpful error messages
- **Data Integrity**: Consistent resource state and relationship validation

**Security Requirements**:
- **Secret Management**: Secure handling of sensitive environment variables
- **Access Controls**: Proper authorization for resource operations
- **Audit Logging**: Complete audit trail for all resource changes
- **Data Protection**: Encryption for sensitive configuration data

**Performance Requirements**:
- **Resource Listing**: <2 seconds for up to 1000 resources
- **Resource Creation**: <3 seconds for complex resource operations
- **Status Queries**: <1 second for resource status information
- **Bulk Operations**: Efficient handling of large-scale operations

## Success Criteria

**Phase Completion Gates**:
- [ ] All resource creation commands functional with proper validation
- [ ] Status commands provide comprehensive and accurate information
- [ ] Environment variable management maintains security for sensitive data
- [ ] Configuration retrieval provides complete resource details
- [ ] Resource operations handle errors gracefully with actionable feedback

**Quality Validation**:
- All acceptance criteria met for each task
- Security review completed for sensitive data handling
- Performance validation under realistic resource loads
- Integration testing with Max platform resource APIs
- User experience validation for common workflows

## Implementation Guidelines

**Resource Management Architecture**:
- Consistent command structure across all resource types
- Unified validation framework for all resource operations
- Centralized error handling and user feedback systems
- Real-time status updates using Phase 2 communication infrastructure

**Security Implementation**:
- Secure secret storage using OS keychain integration
- Encryption for sensitive environment variable data
- Access control validation for all resource operations
- Comprehensive audit logging for security compliance

**Performance Optimization**:
- Efficient resource caching and state management
- Batch operations for improved performance
- Progressive loading for large resource collections
- Optimized queries and filtering capabilities

This phase implements the core resource management functionality that users interact with daily. Focus on creating intuitive, secure, and performant resource management operations that provide excellent user experience while maintaining the highest security standards.