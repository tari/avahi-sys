# avahi-sys

Rust bindings to [Avahi][avahi]'s libavahi-client for zero-configuration
networking with mDNS/DNS-SD.

## Prerequisites

These bindings use [`bindgen`][bindgen] to parse the library's C headers, which
requires that Clang be available at build-time. Refer to the bindgen
documentation for more information.

The Avahi library headers must also be available in your C compiler's default
include search path. In most situations this should be a simple matter of
installing it with your system's package manager.

[avahi]: http://www.avahi.org/
[bindgen]: https://github.com/servo/rust-bindgen

## Internals

Because the Avahi API exposes some types that are not primitives but are part of
the C library, functions and types in the bindings are whitelisted where they
should be exposed. Others like `pollfd` or `timeval` are explicitly referenced
from `libc` so a common type can be used in API consumers rather than
translating to an internal instance of the C library types.
