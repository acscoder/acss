<div align="center">

  <h1><code>Atomic CSS by Rust</code></h1>

  <strong>We re-write Atomic CSS <a href="https://acss.io" target="_blank">acss.io </a> by Rust language.</strong>

  <sub>Built with 🦀🕸 by <a href="https://rustvietnam.com/" target="_blank">Acscoder</a></sub>
</div>

## About

ACSS stands for Atomized CSS. ACSS relies on Atomizer to dynamically generate an Atomic stylesheet from the ACSS classes you're actually using in your project (no unused styles!), or predeclare styles in configuration - it's up to you. ACSS is not opinionated, brings no CSS of its own, and integrates nicely with your favorite task runner. 

## 🚴 Usage
### 🐑 Install wasm-pack 
[Learn more about `wasm-pack` here.](https://rustwasm.github.io/)
```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```
### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/acscoder/acss.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build --target web
```

### 🔬 Test in Localhost with Browsers
If you have python

```
python -m http.server 8000
```
Open http://localhost:8000/

Or we can use some localhost ex: Xampp, Wampp .. to open index.html


## How to use class css

* https://acss.io/quick-start.html
* https://acss.io/reference.html