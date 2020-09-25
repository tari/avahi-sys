# avahi-sys

Rust bindings to [Avahi][avahi]'s libavahi-client for zero-configuration
networking with mDNS/DNS-SD.

For API documentation refer to http://www.avahi.org/doxygen/html/index.html

## Prerequisites

These bindings use [`bindgen`][bindgen] to parse the library's C headers, which
requires that Clang be available at build-time. Refer to the bindgen
documentation for more information.

The Avahi library headers must also be available in your C compiler's default
include search path. In most situations this should be a simple matter of
installing it with your system's package manager.

[avahi]: http://www.avahi.org/
[bindgen]: https://github.com/servo/rust-bindgen
