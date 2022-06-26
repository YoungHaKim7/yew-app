# Rust yew-app

## Yew Tutorial

[Rust html crates\_ Yew\_ Tutorial](https://yew.rs/docs/tutorial)
<br>

<hr>
<br>

# Introduction

- In this hands-on tutorial, we will take a look at how we can use Yew to build web applications. Yew is a modern Rust framework for building front-end web apps using WebAssembly. Yew encourages a reusable, maintainable, and well-structured architecture by leveraging Rust's powerful type system. A large ecosystem of community-created libraries, known in Rust as crates, provide components for commonly-used patterns such as state management. Cargo, the package manager for Rust, allows us to take advantage of the numerous crates available on crates.io, such as Yew.

# What we are going to build

- Rustconf is an intergalactic gathering of the Rust community that happens annually. Rustconf 2020 had a plethora of talks that provided a good amount of information. In this hands-on tutorial, we will be building a web application to help fellow Rustaceans get an overview of the talks and watch them all from one page.

<br>

## Yew Tutorial

[Rust html crates\_ Yew\_ Tutorial](https://yew.rs/docs/tutorial)

<br>

# Setting up

## Prerequisites

To get started, let's make sure we have an up-to-date development environment. We will need the following tools:

- Rust
- trunk
- wasm32-unknown-unknown target, the WASM compiler and build target for Rust.

After installing Rust, you can use Cargo to install trunk by running:

```
cargo install trunk
```

We will also need to add the WASM build target by running:

```
rustup target add wasm32-unknown-unknown
```

<br>

## Start the development server

Run the following command to build and serve the application locally.

```
trunk serve --open
```

<hr>
<br>
# macOS ERROR - error form HTML pipeline

- ERROR ❌ error
  error from HTML pipeline
  <br>
  출처: https://economiceco.tistory.com/14183 [경제PLUS:티스토리]

Fix it by installing wasm-bindgen-cli:

```
cargo install --locked wasm-bindgen-cli
```

# todo app 만들기

[171229_Rustlang Project: Todo App (Yew Framework and Web Assembly)](https://youtu.be/j8EnB7gkygw)
