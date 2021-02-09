- Use eBPF as the low-level representation - eBPF as a common minimal VM description
- parser libraries
  - nom
  - pest
  - lalrpop
  - pom (peg)
  - combine (ll(1))
  - glue

generate a lexing stress test

```
seq 1 100000 | xargs -Inone cat examples/big.bloop-tt | wc -l > examples/super-big.bloop.tt
```

test it with

```
cargo run --manifest-path=src/scratch_driver/Cargo.toml --no-default-features --fe
atures=parallel-tree-walk  -- lex-dump examples/super-big.bloop-tt  > /dev/null
```