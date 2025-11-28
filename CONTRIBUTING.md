# Contributing to LLM Unify

Thank you for your interest in contributing to LLM Unify! This document provides guidelines and information for contributors.

## TPCF Declaration

**Trusted Perimeter Classification Framework (TPCF):** Perimeter 3 (Community Sandbox)

This project operates as a **community-driven open source project** with:
- Public development process
- Open governance model
- Community consensus-driven decisions
- No corporate control or exclusive licensing

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Pull Request Guidelines](#pull-request-guidelines)
- [Coding Standards](#coding-standards)
- [Testing Requirements](#testing-requirements)
- [Documentation](#documentation)
- [License](#license)
- [Path to Maintainership](#path-to-maintainership)

## Code of Conduct

We follow the [Contributor Covenant 2.1](CODE_OF_CONDUCT.md) with CCCP extensions. Please read and follow it.

## Getting Started

### Prerequisites

- **Rust** 1.75+ (latest stable recommended)
- **SQLite** 3.35+
- **just** command runner (optional but recommended)
- **Git** 2.30+

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/Hyperpolymath/llm-unify.git
cd llm-unify

# Build all crates
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings
```

### Repository Structure

```
llm-unify/
├── crates/
│   ├── llm-unify-core/      # Core types and traits
│   ├── llm-unify-storage/   # SQLite persistence
│   ├── llm-unify-parser/    # Import parsers
│   ├── llm-unify-search/    # Full-text search
│   ├── llm-unify-cli/       # Command-line interface
│   └── llm-unify-tui/       # Terminal UI
├── .well-known/             # RFC 9116 metadata
├── SECURITY.md              # Security policy
├── CODE_OF_CONDUCT.md       # Community standards
└── README.md                # Project overview
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

**Branch naming conventions:**
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation updates
- `refactor/` - Code refactoring
- `test/` - Test additions

### 2. Make Changes

- Write code following [Coding Standards](#coding-standards)
- Add tests for new functionality
- Update documentation as needed
- Ensure all tests pass

### 3. Commit Changes

We use **Conventional Commits** format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style (formatting, no logic change)
- `refactor`: Code restructuring (no behavior change)
- `perf`: Performance improvement
- `test`: Adding/updating tests
- `chore`: Maintenance tasks

**Examples:**
```bash
git commit -m "feat(parser): add Claude export parser"
git commit -m "fix(storage): prevent SQL injection in search"
git commit -m "docs(readme): update installation instructions"
```

### 4. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a PR on GitHub.

## Pull Request Guidelines

### Before Submitting

- [ ] Code compiles without errors
- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Documentation updated (if applicable)
- [ ] CHANGELOG.md updated (for significant changes)

### PR Template

```markdown
## Description
Brief description of what this PR does.

## Motivation
Why is this change needed?

## Changes
- Bullet point list of changes

## Testing
How was this tested?

## Checklist
- [ ] Tests pass
- [ ] Code formatted
- [ ] Documentation updated
- [ ] CHANGELOG updated
```

### Review Process

1. **Automated checks** - CI/CD pipeline runs
2. **Code review** - At least one maintainer reviews
3. **Discussion** - Address feedback and questions
4. **Approval** - Maintainer approves PR
5. **Merge** - Squash and merge to main

## Coding Standards

### Rust Style

Follow the official Rust style guide:
- Use `cargo fmt` for automatic formatting
- Follow idiomatic Rust patterns
- Prefer explicit over implicit

### Zero Unsafe Code

**This project has ZERO tolerance for unsafe blocks.**

- **No `unsafe` keyword** allowed in production code
- All memory safety guaranteed by Rust compiler
- Exceptions require maintainer consensus + detailed justification

### Error Handling

```rust
// ✅ Good - Use Result types
pub fn parse_data(input: &str) -> Result<Data, Error> {
    // ...
}

// ❌ Bad - No panics in library code
pub fn parse_data(input: &str) -> Data {
    input.parse().unwrap()  // Never do this!
}
```

### Documentation

```rust
/// Brief one-line summary
///
/// Detailed explanation with examples.
///
/// # Examples
///
/// ```
/// use llm_unify_core::Conversation;
/// let conv = Conversation::new("id".to_string(), "title".to_string(), Provider::ChatGpt);
/// ```
pub fn example_function() {
    // ...
}
```

## Testing Requirements

### Test Coverage

- **Minimum**: 30% (current)
- **Target**: 80% (for RSR Gold)
- **Focus**: Core logic, parsers, storage

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_creation() {
        let conv = Conversation::new(
            "test-id".to_string(),
            "Test Title".to_string(),
            Provider::ChatGpt,
        );
        assert_eq!(conv.id, "test-id");
        assert_eq!(conv.message_count(), 0);
    }

    #[tokio::test]
    async fn test_database_operations() {
        // Test async database operations
    }
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific crate
cargo test -p llm-unify-core

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test '*'
```

## Documentation

### Code Documentation

- All public APIs must have doc comments
- Include examples in doc comments
- Run `cargo doc --open` to preview

### User Documentation

- Update README.md for user-facing changes
- Update relevant .md files in root directory
- Keep CHANGELOG.md current

## License

By contributing, you agree that your contributions will be licensed under:

- **Code**: AGPL-3.0-or-later
- **Data portability**: Palimpsest Protocol

See [LICENSE](LICENSE) and [LICENSE-PALIMPSEST](LICENSE-PALIMPSEST) for details.

## Path to Maintainership

We believe in growing maintainers from contributors.

### Contributor → Regular Contributor
- 3+ merged PRs
- Active in discussions
- Follows contribution guidelines

### Regular Contributor → Committer
- 10+ merged PRs
- Demonstrates deep understanding of codebase
- Helps review others' PRs
- Active for 3+ months

### Committer → Maintainer
- Nominated by existing maintainer
- Consensus approval from maintainer team
- Demonstrated leadership and judgment
- Committed to project long-term

See [MAINTAINERS.md](MAINTAINERS.md) for current maintainer list.

## Questions?

- **General questions**: Open a GitHub Discussion
- **Bug reports**: Open a GitHub Issue
- **Security issues**: See [SECURITY.md](SECURITY.md)

Thank you for contributing to LLM Unify! 🎉
