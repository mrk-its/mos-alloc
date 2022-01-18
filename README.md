# `mos-alloc`

Allocator for `mos-unknown-none` target (https://github.com/mrk-its/rust/tree/mos_target)
and number of utilities for setting max heap size / checking free / used memory



## API Docs
https://docs.rs/mos-alloc

## Example

Full source code in [examples/heap.rs](examples/heap.rs).

``` rust
#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

extern crate alloc;
extern crate mos_alloc;

use alloc::{string::String, vec::Vec};

// ..

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  // ..
  let text = String::from("foo");
  let data = (0..u16).collect::<Vec<_>>();
  ///
}

```

## Running examples

The easiest way is to use provided `devcontainer.json` configuration for vscode:

1. Configure Visual Studio Code with `Remote - Containers` extension
2. Open this project inside devcontainer
3. To build and run `heap` example on mos-sim 6502 simulator do:
    ```
    cargo run --example heap --release
    ```

## License

All source code (including code snippets) is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0][L1])

- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/licenses/MIT][L2])

[L1]: https://www.apache.org/licenses/LICENSE-2.0
[L2]: https://opensource.org/licenses/MIT

at your option.
