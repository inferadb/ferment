# Ferment - Development Guide

A Rust-native terminal UI framework inspired by Bubble Tea / Lip Gloss / Huh Golang ecosystems.

## Quick Reference

```bash
# Build
cargo build

# Test
cargo test                    # All tests
cargo test --doc              # Doc tests only
cargo test <test_name>        # Single test

# Quality
cargo +nightly fmt --all      # Format
cargo clippy --all-targets -- -D warnings  # Lint
```

## Architecture

Ferment follows **The Elm Architecture** (Model-Update-View):

```
┌─────────┐     ┌────────┐     ┌──────────┐
│  Model  │────▶│  View  │────▶│ Terminal │
│ (State) │     │(Render)│     │ (Output) │
└─────────┘     └────────┘     └──────────┘
     ▲                              │
     │         ┌────────┐           │
     └─────────│ Update │◀──────────┘
               │(Logic) │   Events
               └────────┘
```

### Core Traits

- **`Model`** - Application state with init/update/view
- **`Cmd<M>`** - Side effects returning messages
- **`Sub<M>`** - Recurring event sources (timers)
- **`Accessible`** - Screen reader support

### Module Structure

```
src/
├── runtime/           # Core runtime (Program, Cmd, Sub, Model)
│   ├── mod.rs         # Model trait definition
│   ├── program.rs     # Event loop (~500 lines)
│   ├── command.rs     # Cmd<M> implementation
│   ├── subscription.rs # Sub<M> with ID tracking
│   └── accessible.rs  # Accessible trait and helpers
├── components/        # UI components (all implement Model)
│   ├── text_input.rs  # Single-line input
│   ├── text_area.rs   # Multi-line input
│   ├── select.rs      # Single selection
│   ├── multi_select.rs # Multiple selection
│   ├── confirm.rs     # Yes/No
│   ├── list.rs        # Filterable list
│   ├── table.rs       # Data table
│   ├── spinner.rs     # Loading indicator
│   ├── progress.rs    # Progress bar
│   ├── multi_progress.rs # Parallel progress
│   └── viewport.rs    # Scrollable content
├── forms/             # Form system (Huh equivalent)
│   ├── form.rs        # Form container
│   ├── group.rs       # Field groups
│   ├── field.rs       # Field types and builders
│   └── validation.rs  # Validators
├── style/             # Styling (Lip Gloss equivalent)
│   ├── color.rs       # Color definitions
│   ├── text.rs        # Style builder
│   └── border.rs      # Border styles
└── terminal/          # Terminal abstraction
    ├── input.rs       # Event types
    └── backend.rs     # crossterm wrapper
```

## Design Principles

### 1. Elm Architecture Purity

- All state changes through messages
- `update()` returns new state + optional command
- `view()` is a pure function of state
- Side effects only through `Cmd`

### 2. Component Composition

Every component implements `Model`:

```rust
impl Model for MyComponent {
    type Message = MyMsg;
    fn init(&self) -> Option<Cmd<Self::Message>>;
    fn update(&mut self, msg: Self::Message) -> Option<Cmd<Self::Message>>;
    fn view(&self) -> String;
    fn handle_event(&self, event: Event) -> Option<Self::Message>;
}
```

### 3. Builder Pattern APIs

All public APIs use fluent builders:

```rust
TextInput::new()
    .placeholder("Enter name...")
    .prompt("> ")
    .width(40)
```

### 4. Message Naming Convention

Components define `<Name>Msg` enums:

```rust
pub enum TextInputMsg {
    InsertChar(char),
    DeleteBack,
    Submit,
    // ...
}
```

## Charm.sh Alignment

Ferment is inspired by the Charm.sh Go libraries:

| Go Library | Ferment Equivalent | Status            |
| ---------- | ------------------ | ----------------- |
| Bubble Tea | `runtime/`         | Core complete     |
| Bubbles    | `components/`      | 11 components     |
| Huh        | `forms/`           | Basic complete    |
| Lip Gloss  | `style/`           | Needs enhancement |

### Key Differences from Go

1. **Type Safety**: Rust enums for messages vs Go's `interface{}`
2. **Ownership**: `&mut self` in update vs returning new model
3. **Subscriptions**: ID-tracked (better than Go's recreate-each-time)
4. **Accessible**: Explicit trait (Go has no equivalent)

### Gaps to Address (see PLAN.md)

**Styling (Priority 1)**:

- CSS shorthand: `padding(&[2, 4])`
- Layout utilities: `join_horizontal()`, `place()`
- Adaptive colors for light/dark terminals
- Style inheritance and unset methods
- Block dimensions with alignment

**Forms (Priority 2)**:

- Generic field types: `Select::<T>::new()`
- Note field (display-only)
- FilePicker field
- Dynamic content: `title_fn(|| ...)`
- Form layouts (columns, grid)

## Testing

### Unit Tests

Each component has tests in its module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_creation() { ... }
}
```

### Doc Tests

Examples in doc comments are tested:

````rust
/// # Example
/// ```rust
/// let input = TextInput::new().placeholder("Name");
/// ```
````

### Running Tests

```bash
cargo test                    # 105 unit + 28 doc tests
cargo test --doc              # Doc tests only
cargo test text_input         # Specific module
```

## Common Patterns

### Adding a New Component

1. Create `src/components/my_component.rs`
2. Define message enum: `pub enum MyComponentMsg { ... }`
3. Implement `Model` trait
4. Implement `Accessible` trait
5. Add builder methods
6. Export in `src/components/mod.rs`
7. Re-export in `src/lib.rs`
8. Add tests
9. Update README

### Adding a Form Field

1. Create builder struct in `src/forms/field.rs`
2. Add variant to `FieldInner` enum
3. Add variant to `FieldMsg` enum
4. Implement builder methods
5. Handle in `Field::value()`, `is_submitted()`, etc.
6. Export in `src/forms/mod.rs`

### Color Usage

```rust
use ferment::style::Color;

// Named colors
Color::Red
Color::Cyan
Color::BrightBlack  // Dimmed text

// ANSI 256
Color::Ansi256(86)

// True color
Color::Rgb(255, 128, 0)
Color::from_hex("#FF8000")

// Apply
Color::Cyan.to_ansi_fg()  // Foreground
Color::Cyan.to_ansi_bg()  // Background
"\x1b[0m"                 // Reset
```

## Environment Variables

| Variable          | Effect                 |
| ----------------- | ---------------------- |
| `ACCESSIBLE=1`    | Enable accessible mode |
| `NO_COLOR=1`      | Disable colors         |
| `REDUCE_MOTION=1` | Disable animations     |
| `CI=true`         | Non-interactive mode   |

## Debugging Tips

### View Raw Output

```rust
println!("{:?}", component.view());  // Show escape codes
```

### Check Terminal Capabilities

```rust
use ferment::terminal::{is_tty, supports_color, no_color};
```

### Force Non-Interactive

```bash
echo "input" | cargo run --example my_example
```

## Performance Notes

- FPS-based rendering (default 60 FPS)
- Dirty-region tracking minimizes output
- Subscription IDs prevent duplicate timers
- Unicode width caching in text components

## References

- [Bubble Tea](https://github.com/charmbracelet/bubbletea) - Original Go framework
- [Lip Gloss](https://github.com/charmbracelet/lipgloss) - Go styling library
- [Huh](https://github.com/charmbracelet/huh) - Go form library
- [Elm Architecture](https://guide.elm-lang.org/architecture/) - The pattern
- [crossterm](https://docs.rs/crossterm) - Terminal backend
