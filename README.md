# 🚀 rm-node 🚀
###### disclaimer: this project is a joke

### About

JS devs know that NPM fails a lot and usually the solution is to delete `node_modules`, delete the lockfile, and then reinstall. Sadly
`node_modules` is so big it can take minutes to delete. <br>
`rm-node` solves this problem by using the [delete crate](https://github.com/XtremeDevX/delete) powered by [tokio](https://github.com/tokio-rs/tokio) to delete files and folders 2-3x faster!

### How the problem can actually be solved
fix the whole `node_modules` thing and make lockfiles work like they do in Rust

### Features
- async
- cross-platform
- fast

### Installation

You must have Rust and Cargo installed. Install it from [crates.io](https://crates.io/crates/rm-node) <br>
`cargo install rm-node`

### Usage

To delete `node_modules` _and_ any other lockfiles: <br>
**`rm-node --lockfiles`**

To only delete `node_modules`: <br>
`rm-node`

### Bugs and Contributing

Report issues using the [issue tracker](https://github.com/samarmohan/rm-node/issues/new) <br>
Submit PRs on GitHub!

### License
All contributions to this repo will be licensed under the [MIT](https://github.com/samarmohan/rm-node/blob/main/LICENSE) license.
