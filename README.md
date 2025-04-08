# webframe-rs 🧱

A **minimal, builder-style HTML UI framework** for server-side rendering in Rust.  
Use it to compose clean, reusable components (like `<form>`, `<input>`, `<button>`) that render to HTML strings — perfect for use with Askama, Axum, or any Rust web backend.

---

## ✨ Features

- ✅ Component trait for easy `.render()` output
- 🧱 Builder-style API for ergonomic use
- 🎨 Optional themes (Tailwind, Bootstrap, or your own)
- 🧩 Works with Askama and Axum
- 🛠️ Fully customizable: add attributes, classes, raw blocks

---

## 🔧 Example

```rust
use webframe_rs::components::form::Form;
use webframe_rs::components::input::{Input, FieldType};
use webframe_rs::components::button::Button;

let form = Form::new()
    .action("/submit")
    .add(Input::new("email")
        .input_type(FieldType::Email)
        .placeholder("Your email")
        .class("input"))
    .add(Button::new("Send").class("btn btn-primary"));

println!("{}", form.render());
