[Issue link](https://github.com/rust-lang/rustfmt/issues/4334)

Reproduction:

```bash
git clone https://github.com/Veetaha/rustfmt-bugreport.git
cd rustfmt-bugreport

cargo build # works fine
cargo fmt # Error writing files: io error: Failed to find module bar in "." None
```
