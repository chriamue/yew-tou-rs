# yew-tou-rs

Onboarding Tour for [Yew Framework](https://yew.rs) in Rust.

## Links

[![Documentation](https://img.shields.io/badge/docs-rustdoc-blue.svg)](https://chriamue.github.io/yew-tou-rs/doc/yew_tou_rs/)
[![GitHub](https://img.shields.io/badge/github-repo-blue.svg)](https://github.com/chriamue/yew-tou-rs/)
[![Demo](https://img.shields.io/badge/demo-online-brightgreen.svg)](https://chriamue.github.io/yew-tou-rs/)

## Overview

This library provides an onboarding tour for applications built using the Yew framework in Rust.

## Demo

You can see the demo here:

[Live Demo](https://chriamue.github.io/yew-tou-rs/)

## Run Example

```sh
trunk serve
```

Visit [http://localhost:8080](http://localhost:8080) in your browser.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
yew-tou-rs = { git = "https://github.com/chriamue/yew-tou-rs" }
```

Add the following to your `index.html`:

```html
<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/intro.js/7.2.0/introjs.css">
```

See [Intro.js](https://introjs.com/#commercial) for more information about its license.

Add the following to your `App.rs`:

```rust
use yew::prelude::*;
use yew_tou_rs::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let steps = vec![
        TourStep {
            selector: ".h1-step".to_string(),
            content: "This is a title".to_string(),
        }
    ];
    html! {
        <div class="app">
            <h1 class="h1-step">{"Hello, Yew Tou-rs"}</h1>
            <Tour steps={steps} />
        </div>
    }
}
```

## Features

### Storage

If you enable the storage feature, closed tours will not be shown again:

```toml
[dependencies]
yew-tou-rs = { git = "https://github.com/chriamue/yew-tou-rs", features = ["storage"] }
```

### Markdown

If you enable the markdown feature, you can use markdown in the content:

```toml
[dependencies]
yew-tou-rs = { git = "https://github.com/chriamue/yew-tou-rs", features = ["markdown"] }
```

```rust
let steps = vec![
    TourStep {
        selector: ".app".to_string(),
        content: "### Welcome to the tour\n\
            You can find a full demo on [Konnektoren](https://konnektoren.help)".to_string(),
    },
];
```

### Similiar Projects

- [Reactour](https://www.react.tours/) for React
- [Intro.js](https://introjs.com/) for Vanilla JS

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
