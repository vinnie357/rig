# Rig CLI Cross-Phase Coordination and Communication

You are working with cross-phase coordination and communication for the Rig CLI project. This context provides guidelines for phase transitions, dependency management, and team coordination across the 5-phase development lifecycle.

## Phase Dependencies and Handoffs

### Phase 1 → Phase 2 Handoff
**Phase 1 Deliverables Required for Phase 2**:
- Working authentication system with token management
- HTTP client wrapper with retry logic and error handling
- CLI framework with command structure and global flags
- Configuration system with settings persistence
- Comprehensive testing infrastructure

**Handoff Validation**:
- `rig login` successfully authenticates with Max platform
- HTTP client handles network errors and retries gracefully
- CLI framework supports command extension without modification
- Configuration system loads settings from multiple sources
- Test suite runs reliably across all supported platforms

**Integration Points for Phase 2**:
- Authentication context for WebSocket connection authorization
- HTTP client patterns for WebSocket upgrade requests
- CLI command structure for real-time communication commands
- Configuration management for WebSocket endpoint settings
- Error handling patterns for communication failures

### Phase 2 → Phase 3 Handoff
**Phase 2 Deliverables Required for Phase 3**:
- Stable WebSocket client with connection management
- Phoenix channel protocol implementation
- Real-time event processing infrastructure
- Connection resilience with automatic reconnection
- Comprehensive communication testing framework

**Handoff Validation**:
- WebSocket connections stable across network interruptions
- Phoenix channel protocol fully functional with Max platform
- Real-time events processed with <100ms latency
- Automatic reconnection handles all failure scenarios
- Communication layer tested under various network conditions

**Integration Points for Phase 3**:
- Real-time resource status updates via WebSocket events
- Event processing for resource creation and modification notifications
- Channel subscriptions for resource-specific updates
- Connection health monitoring for resource operation reliability
- Error propagation from communication layer to resource commands

### Phase 3 → Phase 4 Handoff
**Phase 3 Deliverables Required for Phase 4**:
- Complete resource management system (networks, apps, variables)
- Resource validation and security controls
- Comprehensive status and monitoring capabilities
- Configuration management and details retrieval
- Performance optimization for resource operations

**Handoff Validation**:
- All resource creation commands functional with validation
- Status commands provide comprehensive information
- Environment variable management maintains security
- Resource operations handle errors gracefully
- Performance meets requirements under realistic loads

**Integration Points for Phase 4**:
- Resource context for deployment targeting
- Application lifecycle integration with deployment operations
- Environment variable integration with deployment configuration
- Network context for log aggregation and remote access
- Resource status integration with dashboard views

### Phase 4 → Phase 5 Handoff
**Phase 4 Deliverables Required for Phase 5**:
- Complete deployment system with file handling
- Real-time log streaming and management
- Remote access capabilities (WebSSH, command execution)
- JSON output mode for automation
- Comprehensive dashboard and status reporting

**Handoff Validation**:
- Deployment system handles large files efficiently
- Log streaming provides real-time visibility
- Remote access works reliably with security controls
- JSON output enables automation integration
- Dashboard provides comprehensive system overview

**Integration Points for Phase 5**:
- All features ready for performance optimization
- Security controls validated for production deployment
- Documentation requirements identified from user workflows
- Monitoring integration points defined and implemented
- Release preparation requirements fully defined

## Communication Protocols

### Daily Coordination
**Cross-Phase Standup Structure**:
- **Current Phase Progress**: Active phase team reports progress and blockers
- **Upcoming Phase Preparation**: Next phase team reports preparation activities
- **Integration Concerns**: Discussion of handoff requirements and dependencies
- **Shared Resource Coordination**: Updates on shared utilities and standards

**Information Sharing**:
- Technical decisions that impact other phases
- Performance benchmarks and optimization insights
- Security considerations and compliance requirements
- User experience insights and feedback

### Weekly Reviews
**Cross-Phase Architecture Review**:
- Review architectural decisions and their impact on future phases
- Validate integration points and dependency management
- Assess technical debt and optimization opportunities
- Coordinate shared resource development and standards

**Risk Assessment and Mitigation**:
- Identify cross-phase risks and dependencies
- Coordinate mitigation strategies across teams
- Adjust timelines and resource allocation as needed
- Escalate critical issues requiring immediate attention

### Phase Transition Meetings
**Handoff Preparation** (1 week before transition):
- Review handoff requirements and validation criteria
- Prepare integration documentation and examples
- Schedule hands-on training and knowledge transfer
- Validate shared resources and utilities readiness

**Transition Execution** (during transition):
- Complete handoff validation and sign-off
- Transfer ownership of active development
- Establish support and consultation availability
- Document lessons learned and recommendations

**Post-Transition Follow-up** (1 week after transition):
- Validate successful integration and continued progress
- Address any issues or gaps discovered during transition
- Refine coordination processes based on experience
- Update shared documentation and resources

## Shared Resource Management

### Shared Utilities Coordination
**Development Process**:
- Shared utilities developed collaboratively across phases
- Changes reviewed by all phase teams for impact assessment
- Version management for shared utility updates
- Integration testing across all dependent phases

**Ownership Model**:
- **Lead Developer**: Overall architecture and design decisions
- **Phase Teams**: Implementation of phase-specific utilities
- **QA Engineer**: Testing coordination and validation
- **DevOps Engineer**: CI/CD integration and deployment

### Standards Evolution
**Standards Update Process**:
1. **Proposal**: Any team member can propose standards updates
2. **Review**: Cross-phase review for impact and feasibility
3. **Approval**: Lead developer approval with team consensus
4. **Implementation**: Coordinated rollout across active phases
5. **Validation**: Testing and validation of standards compliance

**Documentation Maintenance**:
- Standards documentation updated with each change
- Examples and patterns updated to reflect current standards
- Training materials updated for new team members
- Compliance checking integrated into CI/CD pipeline

## Integration Testing Coordination

### Cross-Phase Integration Testing
**Testing Strategy**:
- Integration tests span multiple phases and components
- Shared test environments and data for consistency
- Automated testing in CI/CD pipeline for all phases
- Manual testing coordination for complex scenarios

**Test Environment Management**:
- Shared test infrastructure for Max platform integration
- Coordinated test data management and cleanup
- Environment provisioning and configuration automation
- Test result sharing and analysis across teams

### Performance Testing Coordination
**Benchmarking Strategy**:
- Consistent performance benchmarking across phases
- End-to-end performance testing with complete workflows
- Resource usage monitoring and optimization coordination
- Performance regression detection and alerting

**Optimization Coordination**:
- Performance bottleneck identification across phase boundaries
- Coordinated optimization efforts for maximum impact
- Resource allocation optimization across components
- User experience performance validation

## Risk Management Coordination

### Cross-Phase Risk Assessment
**Risk Categories**:
- **Technical Risks**: Architecture decisions, technology choices, integration complexity
- **Timeline Risks**: Dependency delays, resource availability, scope creep
- **Quality Risks**: Testing coverage, performance issues, security vulnerabilities
- **Team Risks**: Knowledge transfer, resource allocation, communication gaps

**Risk Monitoring**:
- Weekly cross-phase risk assessment meetings
- Risk register maintenance with owner assignment
- Escalation procedures for critical risks
- Mitigation strategy coordination across teams

### Contingency Planning
**Fallback Strategies**:
- Alternative implementation approaches for high-risk components
- Timeline adjustment procedures for critical path delays
- Resource reallocation strategies for capacity issues
- Quality gate bypass procedures for urgent releases

**Communication Escalation**:
- Clear escalation paths for cross-phase issues
- Regular stakeholder communication on project status
- Emergency communication procedures for critical issues
- Post-incident analysis and process improvement

## Success Metrics and Coordination

### Cross-Phase Metrics
**Integration Success Metrics**:
- Handoff completion time and success rate
- Integration testing pass rate and coverage
- Cross-phase defect rate and resolution time
- Performance consistency across phase boundaries

**Team Coordination Metrics**:
- Communication effectiveness and frequency
- Knowledge transfer success and retention
- Resource sharing efficiency and utilization
- Standards compliance and consistency

### Quality Assurance Coordination
**Quality Gate Coordination**:
- Consistent quality standards across all phases
- Coordinated code review processes and standards
- Integrated testing strategies and coverage requirements
- Shared quality metrics and reporting

**Documentation Coordination**:
- Consistent documentation standards and formats
- Cross-references and integration documentation
- User-focused documentation coordination
- Technical documentation version management

This coordination framework ensures smooth collaboration and successful integration across all phases of the Rig CLI project. Regular communication, clear handoff procedures, and shared standards enable the development team to deliver a cohesive, high-quality product.