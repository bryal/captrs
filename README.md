# UNMAINTAINED

I (@bryal) don't have a windows PC anymore (and I don't want one), so I simply don't have any need for this library anymore nor the tools to maintain it. I know there are some people who are currently using this library, so I'd love to hand over maintainership to someone who wants to step up. Whether it's blessing a fork or whatever.

Same goes for X11Cap & dxgcap.

# captrs

Library for cross-platform screen capture in Rust. Uses
[dxgcap](https://github.com/bryal/dxgcap-rs) for capture on Windows
via the Desktop Duplication API, and
[X11Cap](https://github.com/bryal/X11Cap) for capture on Linux via
xlib::XGetImage.

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
