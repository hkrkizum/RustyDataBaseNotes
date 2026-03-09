# Feature Specification: [FEATURE NAME]

**Feature Branch**: `[###-feature-name]`
**Created**: [DATE]
**Status**: Draft
**Input**: User description: "$ARGUMENTS"

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT:
  - User stories must be prioritized as independent user journeys.
  - Use the domain vocabulary from the constitution consistently:
    block, page, database, view, property.
  - Every user story must define how it will be verified independently.
-->

### User Story 1 - [Brief Title] (Priority: P1)

[Describe this user journey in plain language]

**Why this priority**: [Explain the value and why it has this priority level]

**Independent Test**: [Describe how this can be tested independently and what value it proves]

**Acceptance Scenarios**:

1. **Given** [initial state], **When** [action], **Then** [expected outcome]
2. **Given** [initial state], **When** [action], **Then** [expected outcome]

---

### User Story 2 - [Brief Title] (Priority: P2)

[Describe this user journey in plain language]

**Why this priority**: [Explain the value and why it has this priority level]

**Independent Test**: [Describe how this can be tested independently]

**Acceptance Scenarios**:

1. **Given** [initial state], **When** [action], **Then** [expected outcome]

---

### User Story 3 - [Brief Title] (Priority: P3)

[Describe this user journey in plain language]

**Why this priority**: [Explain the value and why it has this priority level]

**Independent Test**: [Describe how this can be tested independently]

**Acceptance Scenarios**:

1. **Given** [initial state], **When** [action], **Then** [expected outcome]

### Edge Cases

- What happens if the application crashes during save, migration, or attachment handling?
- How does the feature behave when local files are missing, locked, or the disk is full?
- How does the feature scale when a page or database contains thousands of blocks or records?
- What happens when a view, property, or nested page references invalid or stale data?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST [core capability]
- **FR-002**: System MUST preserve local-first behavior and MUST NOT require external services
  unless explicitly approved in scope
- **FR-003**: Users MUST be able to recover from common failure cases through clear errors,
  retries, or backup-aware flows
- **FR-004**: System MUST define how affected blocks, pages, databases, views, and properties
  are created, updated, and validated
- **FR-005**: System MUST define persistence behavior, including schema or migration impact

### Key Entities *(include if feature involves data)*

- **[Entity 1]**: [What it represents, key attributes, lifecycle]
- **[Entity 2]**: [What it represents, relationships and invariants]

## Constraints & Compliance *(mandatory)*

- **CC-001 Data Integrity**: [How writes remain atomic, recoverable, and migration-safe]
- **CC-002 Privacy**: [Why the feature does not introduce unintended outbound communication]
- **CC-003 Performance**: [Expected startup, editing, or rendering budget]
- **CC-004 Boundary Types**: [Rust and TypeScript contracts, validation points, error surfaces]
- **CC-005 Testability**: [Which failing tests or executable checks prove the feature first]

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: [Users complete the primary task within a measurable threshold]
- **SC-002**: [The feature remains responsive under a stated local data size]
- **SC-003**: [Failure handling preserves data and communicates recovery steps]
- **SC-004**: [The feature passes required QA commands and story-level verification]
