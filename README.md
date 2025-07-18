# nebz

An immutable non-empty bytes container.

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).

For more details, see [docs.rs/nebz](https://docs.rs/nebz).

## Safety Notice

This crate uses **unsafe code** for performance optimization in methods like `first()`, `last()`, `split_first()`, and `split_last()`. All unsafe blocks are:

- Thoroughly documented with safety justifications
- Protected by `NonEmptyBz<T>`'s non-empty guarantee
- **Verified with Miri** - passes all undefined behavior detection tests

Run `cargo +nightly miri test` to verify the safety.
