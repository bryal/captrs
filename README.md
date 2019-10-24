# captrs

Cross-platform screen capture in Rust. Uses
[dxgcap](https://github.com/bryal/dxgcap-rs) for capture on Windows
via the Desktop Duplication API, and
[X11Cap](https://github.com/bryal/X11Cap) for capture on Linux via
xlib::XGetImage.
