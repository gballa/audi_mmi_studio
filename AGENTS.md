# Architecture & Quality Standards

## Structure
- Follow strict Clean Architecture: Transports/Handlers -> Services -> Repositories.
- Domain logic must remain independent of external frameworks and database drivers.
- All outbound network calls and database queries require explicit timeouts and context propagation.

## Quality & Error Handling
- Strict type checking enabled; no any types.
- Throw typed domain errors with error codes instead of generic strings.
- Test coverage target: >= 80% for domain logic.

## Verification
- Run tests and linter before marking tasks complete.
