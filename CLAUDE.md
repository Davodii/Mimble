# Mimble

A tree-walking interpreter for a custom scripting language, written in Rust. CS310 school project.

## Build & Run

```bash
cargo build
cargo run --bin repl          # Interactive REPL
cargo run --bin file <file>   # Execute a .mbl source file
```

## Tests

```bash
cargo test
cargo test -- --nocapture     # Show stdout
```

Test files are `.mbl` files in `tests/{lexer,parser,analyser,evaluator}/`. The harness in `tests/integration.rs` discovers and runs them automatically.

Test file conventions:
- `# expect: <value>` — assert successful output
- `# error: <ErrorType>` — assert a specific error is raised

## Architecture

Source code flows through four sequential stages:

```
Source → [Lexer] → Tokens → [Parser] → AST → [Analyser] → [Evaluator] → Value
```

Each stage is gated: if the previous stage produced errors, execution stops. Errors are collected in a `DiagnosticsSink` inside `Context` rather than returned immediately.

### Key modules

| Path | Role |
|------|------|
| `src/lib.rs` | `Interpreter` — orchestrates the full pipeline |
| `src/lexer/` | Tokenisation; `token.rs` defines all token types |
| `src/parser/` | Recursive-descent parser producing an AST; `ast.rs` defines node types |
| `src/analyser/` | Semantic analysis: variable resolution, type checking |
| `src/evaluator/` | Tree-walking interpreter; `environment.rs` manages scopes, `value.rs` defines runtime values |
| `src/common/` | Shared utilities: `Context`, `SymbolPool`, `DiagnosticsSink`, `Span`, type system |
| `src/stdlib/` | Built-in functions (`print`, `len`) registered via `GlobalsBuilder` |
| `src/tracer/` | Optional execution tracing (`DummyTracer`, `ConsoleTracer`, `TraceCollector`) |

### Important patterns

- **Symbol interning** — all identifiers are interned in `SymbolPool`; compare symbols by ID, not string.
- **`Spanned<T>`** — every AST node carries a source location for precise error messages.
- **`Context`** — passed through all stages; holds the symbol pool and diagnostics sink.
- **`TrackedValue`** — runtime values include a `DataSource` tag (Variable, ArraySlot, Literal, etc.) used by the tracer.
- **Environment chains** — scopes are linked `HashMap<Symbol, TrackedValue>` nodes with a `parent` pointer.
- **Native functions** — `Arc`-wrapped closures registered through `GlobalsBuilder`; use `Type::Any` for polymorphic builtins.

## Language Features

- Types: `integer`, `float`, `string`, `bool`, `nil`, arrays, functions
- `let x = value` / `let x: integer = value` (type annotations optional)
- Operators: `+` `-` `*` `/` `%`, comparisons, `and` `or` `not`
- `if / elif / else`, `while`, `break`, `continue`
- `func name(params) -> ReturnType { body }`
- Array literals `[1, 2, 3]`, indexing `arr[i]` (read and write)
- `do ... end` blocks
- Integer→float promotion on mixed arithmetic; `+` coerces to string when one operand is a string

## Dependencies

- `serde` + `serde_json` — serialisation (used for tracing output)
