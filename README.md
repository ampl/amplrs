# amplrs

Safe Rust bindings to [AMPL](https://ampl.com), the algebraic modeling language for mathematical
optimization. `amplrs` wraps the AMPL C API directly: it lets you build, solve, and inspect AMPL models
from Rust, with model generation and solver interaction handled entirely by AMPL.

## Requirements

- A local AMPL installation (+ license) to actually run models. A free
  [Community Edition license](https://ampl.com/ce/) gives free use of AMPL with open-source solvers
  (e.g. HiGHS, CBC). `amplrs` looks for the `ampl` executable on `PATH` by default, or you can point at it
  explicitly with `Environment`/`Ampl::new_with_env`.
- `libclang`, used by [bindgen](https://rust-lang.github.io/rust-bindgen/) to generate the FFI bindings at
  build time. Usually already available on Linux/macOS toolchains; on Windows set `LIBCLANG_PATH` if it
  isn't auto-detected.

## Installation

```bash
cargo add amplrs
```

## Quick example

```rust
use amplrs::Ampl;

fn main() {
    let mut ampl = Ampl::new();
    ampl.set_option("solver", "highs");

    ampl.read("models/diet/diet.mod");
    ampl.read_data("models/diet/diet.dat");
    ampl.solve("", "");

    let total_cost = ampl.get_objective("Total_Cost");
    println!("Objective is: {}", total_cost.value());
}
```

See [`examples/`](examples/) for complete, runnable programs covering data assignment
([`firstexample.rs`](examples/firstexample.rs)), building a model's data entirely from Rust
([`dietexample.rs`](examples/dietexample.rs), [`multidimensionalexample.rs`](examples/multidimensionalexample.rs)),
options ([`optionsexample.rs`](examples/optionsexample.rs)), error handling
([`writemodel.rs`](examples/writemodel.rs)), and a solve heuristic
([`trackingmodel.rs`](examples/trackingmodel.rs)):

```bash
cargo run --example firstexample -- highs
```
