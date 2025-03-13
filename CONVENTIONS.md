functional-programming/CONVENTIONS.md
---

# Data Nexus Project Conventions

## Core Principles

- **Data Integrity First**: Ensure accuracy and consistency in all data operations
- **Clarity Over Cleverness**: Prioritize readable/maintainable code over optimization
- **Semantic Organization**: Structure data and code to reflect real-world relationships
- **Knowledge Connectivity**: Explicitly document relationships between data entities
- **Provenance Tracking**: Maintain clear lineage for all data transformations

## Rust Development Standards

### Code Quality

- Adhere to Rustfmt formatting and Clippy lints
- Prefer `#![forbid(unsafe_code)]` unless explicitly required
- Document all public APIs with rustdoc including examples
- Keep functions under 25 lines with single responsibility
- Use `rustix` instead of raw libc for system interactions

### Data Handling

1. **Validation**
   - Use `validator` crate with custom derive rules
   - Apply schema validation before data ingestion
   - Implement `Sanitize` trait for all external inputs

2. **Transformation**
   - Leverage `Iterator` combinators for pipeline processing
   - Use `serde` with schema-aware deserialization
   - Create idempotent transformation functions

3. **Storage**
   - Tag all data entities with metadata:

     ```rust
     pub struct DataEntity<T> {
         provenance: Vec<ChangeRecord>,
         version: SemanticVersion,
         created: DateTime<Utc>,
         modified: DateTime<Utc>,
         content: T
     }
     ```

   - Use `Arc<str>`/`Arc<[u8]>` for large immutable datasets

### Knowledge Modeling

- Represent relationships using graph primitives:

  ```rust
  pub struct KnowledgeEdge {
      source: NodeId,
      target: NodeId,
      relationship: RelationshipType,
      weight: OrderedFloat<f64>,
      evidence: Vec<EvidenceSource>
  }
  ```

- Implement trait-based polymorphism:

  ```rust
  pub trait KnowledgeUnit {
      fn connections(&self) -> Vec<KnowledgeEdge>;
      fn contextualize(&self, context: &QueryContext) -> Interpretation;
  }
  ```

### Performance

- Use `rayon` for data parallelism where applicable
- Profile with `criterion` for critical paths
- Apply LRU caching with `lru` crate for frequent queries
- Precompute connectivity metrics for knowledge graphs

### Error Handling

- Use `thiserror` for library error types
- Use `anyhow` for application-level errors
- Contextualize errors with `.inspect()` and `.tap()`
- Implement automatic error conversion traits:

  ```rust
  impl From<DataError> for KnowledgeError {
      fn from(e: DataError) -> Self {
          KnowledgeError::IntegrationFailure(e.to_string())
      }
  }
  ```

## Security Practices

- Validate all inputs with whitelist patterns
- Sanitize outputs for different contexts (HTML/SQL/CLI)
- Use `secrecy` crate for sensitive data
- Implement rate limiting on data ingestion
- Audit dependencies with `cargo-audit`

## Testing Standards

- **Unit Tests**: Verify core data operations in isolation
- **Property Tests**: Use `proptest` for data validation rules
- **Integration Tests**: Test cross-component data flows
- **Fuzz Tests**: Implement with `libfuzzer-sys` for parsers
- **Benchmarks**: Maintain performance baselines

## Documentation

- Use rustdoc with `--document-private-items`
- Maintain EXAMPLES.md with data workflow diagrams
- Keep DATA_SCHEMA.md updated with type relationships
- Document all knowledge graph relationships in SCHEMA.dot

## Project Structure

```
src/
├── data/            # Core data types
├── knowledge/       # Relationship modeling
├── pipelines/       # Data transformation flows
├── storage/         # Persistence implementations
├── validation/      # Schemas and sanitization
└── analysis/        # Connectivity algorithms
```

## CI/CD Requirements

- All builds must pass Clippy `pedantic` checks
- 90%+ test coverage measured by `tarpaulin`
- WASM cross-compilation check for core logic
- Documentation build verification
- Security audit on schedule

## Knowledge Graph Specifics

1. Use RDF-compatible triple storage format
2. Implement SPARQL-like query interface
3. Maintain inverse indices for bidirectional traversal
4. Support multiple relationship weighting systems
5. Enable temporal versioning of graph connections

Adherence to these conventions ensures consistent implementation of our core mission to organize and connect knowledge effectively while maintaining Rust's safety guarantees.
