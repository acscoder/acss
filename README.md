<div align="center">

  <h1><code>Atomic CSS by Rust</code></h1>

  <strong>We re-write Atomic CSS <a href="https://acss.io" target="_blank">acss.io </a> by Rust language.</strong>

  <sub>Built with 🦀🕸 by <a href="https://rustvietnam.com/" target="_blank">Acscoder</a></sub>
</div>

## About

ACSS stands for Atomized CSS. ACSS relies on Atomizer to dynamically generate an Atomic stylesheet from the ACSS classes you're actually using in your project (no unused styles!), or predeclare styles in configuration - it's up to you. ACSS is not opinionated, brings no CSS of its own, and integrates nicely with your favorite task runner. 

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.