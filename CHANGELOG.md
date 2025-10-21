# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-10-21

### Added
- Initial implementation of Shishutsukan API client in Rust
- Support for expense management operations:
  - `add_expense` - Add new expense entries
  - `get_expenses` - Retrieve list of expenses
  - `delete_expense` - Delete expense by ID
- Support for genre management operations:
  - `get_genres` - Retrieve list of genres
  - `add_genre` - Add new genre
  - `delete_genre` - Delete genre by ID
- Data models:
  - `Expense` - Expense data without ID
  - `ExpenseWithId` - Expense data with ID
  - `Genre` - Genre data without ID
  - `GenreWithId` - Genre data with ID
  - `ApiMessage` - API response message
- Custom error types with `ShishutsukanError`
- Comprehensive documentation with examples
- Basic usage example in `examples/basic_usage.rs`
- Unit tests for core functionality

### Dependencies
- reqwest 0.12 - HTTP client
- serde 1.0 - Serialization/deserialization
- tokio 1.0 - Async runtime
- thiserror 2.0 - Error handling

[0.1.0]: https://github.com/petitstrawberry/shishutsukan-rs/releases/tag/v0.1.0
