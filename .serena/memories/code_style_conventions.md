# Code Style & Conventions

## Rust Formatting (.rustfmt.toml)
The project uses rustfmt with the following configuration:
- **Style Edition**: 2024
- **Comment Width**: 100 characters
- **Imports**: Group by std/external/crate, module-level granularity
- **Newline Style**: Unix (LF)
- **Comments**: Normalized and wrapped
- **Match Blocks**: Trailing comma
- **Small Heuristics**: MAX (prefer single-line formatting when possible)

## Naming Conventions
- **Types/Structs/Enums**: PascalCase (e.g., `TextInput`, `SpinnerStyle`)
- **Functions/Methods**: snake_case (e.g., `handle_event`, `with_placeholder`)
- **Constants**: SCREAMING_SNAKE_CASE
- **Module names**: snake_case
- **Message types**: Usually an enum named `Msg` or `Message`

## Code Organization
- Public API exposed through `src/lib.rs` via module re-exports
- Each component in its own file under `src/components/`
- Related functionality grouped into modules (runtime, style, forms, etc.)

## Component Pattern
Components follow a builder pattern for construction:
```rust
let input = TextInput::new()
    .placeholder("Enter text...")
    .prompt("> ");
```

## Model Trait Pattern
Components implement the `Model` trait:
```rust
impl Model for MyComponent {
    type Message = MyMsg;
    
    fn init(&self) -> Option<Cmd<Self::Message>> { ... }
    fn update(&mut self, msg: Self::Message) -> Option<Cmd<Self::Message>> { ... }
    fn view(&self) -> String { ... }
    fn handle_event(&self, event: Event) -> Option<Self::Message> { ... }
}
```

## Documentation
- Public items should have doc comments (`///`)
- Modules should have module-level documentation (`//!`)
- Examples in doc comments when helpful

## Error Handling
- Use `Result` types for fallible operations
- Prefer `?` operator for error propagation
- Use `Option` for optional values

## Dependencies
- Minimize external dependencies
- Prefer crossterm for terminal operations
- Use standard library features when possible
