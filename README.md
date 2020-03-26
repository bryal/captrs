# captrs

Cross-platform screen capture in Rust. Uses
[dxgcap](https://github.com/bryal/dxgcap-rs) for capture on Windows
via the Desktop Duplication API, and
[X11Cap](https://github.com/bryal/X11Cap) for capture on Linux via
xlib::XGetImage.

## Building & Installing

Install Rust (and Cargo) via your distribution's package manager or [rustup](https://rustup.rs/), then simply build with command `cargo build`, and install to the default cargo installation location with `cargo install`.

## License

AGPLv3

Copyright (C) 2019  Johan Johansson

This program is free software: you can redistribute it and/or
modify it under the terms of the GNU Affero General Public License
as published by the Free Software Foundation, either version 3 of
the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
Affero General Public License for more details.

See [LICENSE](./LICENSE)
