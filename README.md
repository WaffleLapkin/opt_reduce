<div align="center">
  <h1>opt_reduce</h1>

  <a href="https://crates.io/crates/opt_reduce">
    <img alt="crates.io" src="https://img.shields.io/crates/v/opt_reduce">
  </a>    
  <a href="https://docs.rs/opt_reduce">
    <img alt="documentation (docs.rs)" src="https://docs.rs/opt_reduce/badge.svg">
  </a>
  <a href="LICENSE">
    <img alt="LICENSE (MIT)" src="https://img.shields.io/badge/license-MIT-brightgreen.svg">
  </a>
</div>

This crate provides a `reduce` function for `Option<_>` that allows to
merge two options together.

This method was previously proposed for addition to `std` two times but both
PRs were closed:
1. [#84695][first PR]
2. [#87036][second PR]

[first PR]: https://github.com/rust-lang/rust/pull/84695
[second PR]: https://github.com/rust-lang/rust/pull/87036

---

```toml
opt_reduce = "1"
```
_Compiler support: requires rustc 1.31+_.
