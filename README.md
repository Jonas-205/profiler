# Profiler
A rust tool that helps you to easily profile your code in three diffrent scopes
Can be viewed in chromium based browsers at chrome://tracing/

As a lot of tracing can be slow, this crate can be completly disabled using the disable feature

# Examples
```rust
use profiler;

profiler::time_init!();  // Will time the current scope with funtion name
profiler::time_init!("str");  // Will time the current scope with funtion name + String

profiler::time_run!();  // Will time the current scope with funtion name
profiler::time_run!("str");  // Will time the current scope with funtion name + String

profiler::time_free!();  // Will time the current scope with funtion name
profiler::time_free!("str");  // Will time the current scope with funtion name + String
```

# Getting Started
```bash
cargo run --example profiler
```