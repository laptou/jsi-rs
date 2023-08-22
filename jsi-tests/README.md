# `js-tests`

This crate contains tests for `jsi-rs`. The tests are run against
[Hermes](https://github.com/facebook/hermes), which is Facebook's reference
implementation of JavaScript for React Native.

## Building and running tests

Hermes is a Git submodule of this repository, so it should be cloned
automatically when you clone this repo. The initial build of this crate will
take a while because it builds Hermes first.

Once that build finishes, you can run `cargo test` as normal.
