# Cargo Hook
Adds a git hook to your repository, which will format, lint and test your rust code, and ask you to 
fix any problems before you commit.

```bash
cargo install cargo-hook
cargo hook
```
```
---------------------------
- ✨  Running clippy   ✨ -
---------------------------
    Checking cargo-hook v0.1.0 (/Users/jacko/src/cargo-hook)
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
✅
---------------------------
- ✨ Running formatter ✨ -
---------------------------
✅
---------------------------
- ✨   Running tests   ✨ -
---------------------------
   Compiling cargo-hook v0.1.0 (/Users/jacko/src/cargo-hook)
    Finished test [unoptimized + debuginfo] target(s) in 0.32s
     Running unittests src/main.rs (target/debug/deps/cargo_hook-23cd2685ff070753)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

✅
--------------------------------------
- 🎉 linted, formatted and tested 🎉 -
--------------------------------------
```