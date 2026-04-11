# cuda-schema

Schema validation — type checking, constraints, JSON schema for agent messages

Part of the Cocapn data layer — structured data handling, indexing, and analytics.

## What It Does

### Key Types

- `FieldDef` — core data structure
- `ValidationError` — core data structure
- `Schema` — core data structure

## Quick Start

```bash
# Clone
git clone https://github.com/Lucineer/cuda-schema.git
cd cuda-schema

# Build
cargo build

# Run tests
cargo test
```

## Usage

```rust
use cuda_schema::*;

// See src/lib.rs for full API
// 10 unit tests included
```

### Available Implementations

- `FieldDef` — see source for methods
- `Schema` — see source for methods

## Testing

```bash
cargo test
```

10 unit tests covering core functionality.

## Architecture

This crate is part of the **Cocapn Fleet** — a git-native multi-agent ecosystem.

- **Category**: data
- **Language**: Rust
- **Dependencies**: See `Cargo.toml`
- **Status**: Active development

## Related Crates

- [cuda-pattern-match](https://github.com/Lucineer/cuda-pattern-match)
- [cuda-anomaly](https://github.com/Lucineer/cuda-anomaly)
- [cuda-stats](https://github.com/Lucineer/cuda-stats)
- [cuda-serde](https://github.com/Lucineer/cuda-serde)
- [cuda-index](https://github.com/Lucineer/cuda-index)
- [cuda-cache](https://github.com/Lucineer/cuda-cache)
- [cuda-compression](https://github.com/Lucineer/cuda-compression)
- [cuda-hash](https://github.com/Lucineer/cuda-hash)
- [cuda-sort](https://github.com/Lucineer/cuda-sort)

## Fleet Position

```
Casey (Captain)
├── JetsonClaw1 (Lucineer realm — hardware, low-level systems, fleet infrastructure)
├── Oracle1 (SuperInstance — lighthouse, architecture, consensus)
└── Babel (SuperInstance — multilingual scout)
```

## Contributing

This is a fleet vessel component. Fork it, improve it, push a bottle to `message-in-a-bottle/for-jetsonclaw1/`.

## License

MIT

---

*Built by JetsonClaw1 — part of the Cocapn fleet*
*See [cocapn-fleet-readme](https://github.com/Lucineer/cocapn-fleet-readme) for the full fleet roadmap*
