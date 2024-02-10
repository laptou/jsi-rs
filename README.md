# `jsi-rs`

This library makes it possible to write React Native JSI modules in Rust.

For an example, check out the `example` folder.

## Getting Started

1. Clone this repo
2. Run `git submodule init`
3. Run `git submodule update`, this will ensure that all the vendor dependencies are cloned locally
4. Make sure you have Ninja installed locally, which is necessary for building Hermes. You can find instructions [here](https://github.com/ninja-build/ninja/wiki/Pre-built-Ninja-packages). On macOS, you can install it with `brew install ninja`
5. Install dependencies for the example app: `cd example && yarn install`
6. Run the example app on android with `yarn android`

> NOTE: Make sure that you have not installed rust with homebrew on mac, use the `rustup` toolchain instead.

## Contributing

I wrote this code in winter 2022 as part of another project. A few months later,
I have decided to release it to the world. However, I'm not planning to maintain
it unless I encounter another project that requires it, so for now, the code is
given to you as-is. Feel free to contribute PRs that would improve the API or
stability of the library.

## Safety

Right now, this library is quite `unsafe`.

## Copyright / license

Copyright Ibiyemi Abiodun. MIT License.
