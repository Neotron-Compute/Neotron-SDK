# Neotron SDK

The Neotron SDK defines the API that applications receive when they run on the
[Neotron OS](https://github.com/neotron-compute/neotron-os).

You should use this crate when writing applications that run on Neotron OS.

This SDK attempts to detect targets that support UNIX or Windows, and implements
some code to talk to the appropriate UNIX or Windows API. This allows some level
of portable, mainly to support application testing on those OSes.

On a *bare-metal* target (i.e. where the OS is `none`), the SDK expects the
Neotron OS to pass the callback table to the entry point (`app_entry()`). Once
initialised, the SDK then expects you application to provide an `extern "C"`
`no-mangle` function called `neotron_main`, which the SDK will call.

## Samples

Some [sample](./samples/README.md) applications are provided with this SDK.

## Changelog

See [CHANGELOG.md](./CHANGELOG.md)

## Licence

Copyright (c) The Neotron Developers, 2024

Licensed under either [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE) at
your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be licensed as above, without any
additional terms or conditions.

