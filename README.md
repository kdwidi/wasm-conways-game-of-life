<div align="center">

  <h1><code>wasm-pack-template</code></h1>
  <sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## About

This is the final result source code of [ Conway's Game of Life Rust and WebAssembly ][conways-link] tutorial. Instead of using `enum`, this source code implements [`FixedBitSet`][fixedbitset-link] which only require `1 bit` memory to store a cell data over `1 byte` in `enum`.

## Usage

### Build
```
wasm-pack build
```

### Run
```
cd www/
npm run start
```

[fixedbitset-link]: https://docs.rs/fixedbitset/latest/fixedbitset/index.html
[conways-link]: https://rustwasm.github.io/docs/book/game-of-life/introduction.html
## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)