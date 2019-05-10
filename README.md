# bop
> Parsee don't cry

![](https://img.shields.io/badge/development%20status-not%20working-red.svg?style=for-the-badge&logo=github)

Bop is a general purpose parser generator written in Rust that is inspired by [pest](https://pest.rs), [rocket.rs](https://rocket.rs) and [nom-peg](https://github.com/rust-bakery/nom-peg), a PEG parser built on top of nom.

The only difference is this is code-syntax file separated, binding the logic layer and syntax rules via Rust's macro annotations.