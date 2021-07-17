This crate provides a `reduce` function for [`Option<_>`] that allows to
merge two options together.

This method was previously proposed for addition to `std` two times but both
PRs were closed.
1. Add `Option::merge` under `option_merge` feature gate [#84695][first PR]
2. Added `Option::reduce` [#87036][second PR]

[first PR]: https://github.com/rust-lang/rust/pull/84695
[second PR]: https://github.com/rust-lang/rust/pull/87036

```toml
opt_reduce = "1"
```
_Compiler support: requires rustc 1.31+_.
