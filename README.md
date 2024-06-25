# yew-tou-rs

Onboarding Tour for [Yew Framework](https://yew.rs) in Rust.

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
<link rel="stylesheet" href="https://unpkg.com/intro.js/introjs.css">
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

If you enable the storage feature, closed tours will not be shown again:

```toml
[dependencies]
yew-tou-rs = { git = "https://github.com/chriamue/yew-tou-rs" , features = ["storage"] }
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
