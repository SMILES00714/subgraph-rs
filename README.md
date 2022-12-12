# `subgraph-rs`

Write Subgraphs for The Graph protocol in Rust 🦀.

## Roadmap

- [x] Test-bench for executing mapping modules for rapid development
- [x] Hello world mapping with manual deployment
- [x] Manual deployment to local subgraph node
- [x] MVP Subgraph API coverage (at least enough to create a subgraph for GRT)
- [ ] Basic `cargo-subgraph` to local node
- [ ] `cargo-subgraph` to Subgraph Studio
- [ ] Procedural attribute macro for type-safe mapping handlers
    - Pointers conversion, `nomangle`, WASM export, etc.
    - Set up panic hook
- [ ] Mock set of host functions to support `cargo test`
- [ ] **FIRST RELEASE**

## Future

- [ ] Add support for `log` and `tracing` frontends.
    - Feature gated
    - Attach logger/subscriber in mapping handler proc-macro
- [ ] Procedural derive macro for contract ABI types
    - Function calls with inputs and outputs
    - Event data conversions
    - Piggy-back on `serde`?
    - Generate code in build script from contract ABI
- [ ] Procedural derive macros for entity storage types
    - Piggy-back on `serde`?
    - Generate code in build script from `subgraph.graphql`
- [ ] Templating features for `subgraph.yaml` and `subgraph.graphql`
    - Expose things like environment variables
    - Maybe support multiple environments with `subgraph.${environment}.yaml`
