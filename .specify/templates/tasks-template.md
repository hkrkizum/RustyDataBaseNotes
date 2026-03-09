---

description: "Task list template for feature implementation"
---

# Tasks: [FEATURE NAME]

**Input**: Design documents from `/specs/[###-feature-name]/`
**Prerequisites**: plan.md (required), spec.md (required), research.md,
data-model.md, contracts/

**Tests**: Tests are MANDATORY. Each user story must start with failing tests or an
equivalent executable verification task before implementation tasks appear.

**Organization**: Tasks are grouped by user story so each story can be implemented,
tested, and reviewed independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel if they touch different files and have no dependency
- **[Story]**: Which user story this task belongs to, for example `US1`
- Include exact file paths in every task description

## Path Conventions

- **Desktop app**: `src/` for frontend, `src-tauri/src/` for Rust backend,
  `src-tauri/migrations/` for schema changes
- **Shared tests**: `tests/integration/`, `tests/e2e/`, or framework-native test paths
- Adjust paths to the concrete structure declared in `plan.md`

<!--
  IMPORTANT:
  - Replace all sample tasks with concrete tasks derived from spec.md and plan.md.
  - Preserve the phase structure unless the feature has a justified reason not to.
  - Every story must include test tasks, implementation tasks, and documentation or
    QA closeout tasks required by the constitution.
-->

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish only the minimum scaffolding required for the feature

- [ ] T001 Create or confirm feature directories from `plan.md`
- [ ] T002 Configure dependencies in `package.json`, `pnpm-lock.yaml`, or Cargo manifests
- [ ] T003 [P] Add or update lint and format commands used by this feature

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST exist before any user story lands

**⚠️ CRITICAL**: No user story work begins until this phase is complete

- [ ] T004 Define or update storage schema and migrations in `src-tauri/migrations/`
- [ ] T005 [P] Establish typed IPC contracts across `src/` and `src-tauri/src/ipc/`
- [ ] T006 [P] Add backup, recovery, or failure-handling hooks required by the feature
- [ ] T007 [P] Add domain errors and validation rules without `unwrap()` or `panic!()`
- [ ] T008 [P] Guard against unintended outbound network behavior if the feature touches I/O
- [ ] T009 Document bounded contexts, invariants, and public API doc comment impact

**Checkpoint**: Foundation is ready and user stories can proceed

---

## Phase 3: User Story 1 - [Title] (Priority: P1) 🎯 MVP

**Goal**: [Brief description of the value delivered]

**Independent Test**: [How to prove this story works on its own]

### Tests for User Story 1

- [ ] T010 [P] [US1] Add failing integration or component test in [path]
- [ ] T011 [P] [US1] Add failing backend or contract test in [path]

### Implementation for User Story 1

- [ ] T012 [P] [US1] Implement frontend changes in [path]
- [ ] T013 [P] [US1] Implement Rust domain or IPC changes in [path]
- [ ] T014 [US1] Implement persistence, validation, and error handling in [path]
- [ ] T015 [US1] Add or update `///` documentation and user-facing copy in [path]
- [ ] T016 [US1] Run and record story-specific QA commands

**Checkpoint**: User Story 1 is fully functional and independently verifiable

---

## Phase 4: User Story 2 - [Title] (Priority: P2)

**Goal**: [Brief description of the value delivered]

**Independent Test**: [How to prove this story works on its own]

### Tests for User Story 2

- [ ] T017 [P] [US2] Add failing integration or component test in [path]
- [ ] T018 [P] [US2] Add failing backend or contract test in [path]

### Implementation for User Story 2

- [ ] T019 [P] [US2] Implement frontend changes in [path]
- [ ] T020 [P] [US2] Implement Rust domain or IPC changes in [path]
- [ ] T021 [US2] Implement persistence, validation, and error handling in [path]
- [ ] T022 [US2] Add or update `///` documentation and user-facing copy in [path]
- [ ] T023 [US2] Run and record story-specific QA commands

**Checkpoint**: User Stories 1 and 2 both work independently

---

## Phase 5: User Story 3 - [Title] (Priority: P3)

**Goal**: [Brief description of the value delivered]

**Independent Test**: [How to prove this story works on its own]

### Tests for User Story 3

- [ ] T024 [P] [US3] Add failing integration or component test in [path]
- [ ] T025 [P] [US3] Add failing backend or contract test in [path]

### Implementation for User Story 3

- [ ] T026 [P] [US3] Implement frontend changes in [path]
- [ ] T027 [P] [US3] Implement Rust domain or IPC changes in [path]
- [ ] T028 [US3] Implement persistence, validation, and error handling in [path]
- [ ] T029 [US3] Add or update `///` documentation and user-facing copy in [path]
- [ ] T030 [US3] Run and record story-specific QA commands

**Checkpoint**: All planned user stories are independently functional

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Work that spans multiple user stories

- [ ] TXXX [P] Update shared documentation such as `README.md` or `quickstart.md`
- [ ] TXXX Remove temporary code and simplify abstractions that are no longer needed
- [ ] TXXX Verify performance targets for large pages, databases, or timeline views
- [ ] TXXX [P] Add coverage for recovery, migration, and backup scenarios
- [ ] TXXX Run full QA: formatting, lint, tests, docs, and frontend checks

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup**: Starts immediately
- **Foundational**: Depends on Setup and blocks all user stories
- **User Stories**: Start only after Foundational completes
- **Polish**: Starts after all targeted user stories are complete

### Within Each User Story

- Tests MUST be written first and MUST fail before implementation
- Frontend and backend tasks can run in parallel only when contracts are stable
- Persistence and migration work must complete before story sign-off
- Documentation and QA tasks are required for story completion

### Parallel Opportunities

- Tasks marked `[P]` can run in parallel
- Different user stories can run in parallel only after foundational tasks complete
- Recovery tests, UI tests, and domain tests may run in parallel when file ownership is separate

---

## Implementation Strategy

### MVP First

1. Complete Setup
2. Complete Foundational
3. Complete User Story 1
4. Validate User Story 1 independently before expanding scope

### Incremental Delivery

1. Ship the smallest useful story first
2. Re-run constitution-aligned QA after each story
3. Add the next story only after the previous one remains green

---

## Notes

- Do not create tasks that hide file paths or expected outputs
- Do not mark tests as optional
- Do not bypass data integrity, backup, or migration work for convenience
- Do not introduce `unsafe`, `unwrap()`, `expect()`, or `panic!()` tasks
