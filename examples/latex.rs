use std::path::PathBuf;

use rocket::tokio;

use mokareads_core::latex::{build, BuildType};

const MD: &str = r#"---
title: Tester
description: This is for testing
author: Mustafif Khan
date: 01-01-2023
tags: Rust, C
icon: devicon-rust-plain
---

# This is for testing
hello this is for testing

```rust
println!("This is rust");
```
"#;

#[tokio::main]
async fn main() {
    build(
        MD,
        "latex_example",
        PathBuf::from("examples"),
        BuildType::TeX,
    )
    .await
    .unwrap();
    build(
        MD,
        "latex_example",
        PathBuf::from("examples"),
        BuildType::PDF,
    )
    .await
    .unwrap();
    build(
        MD,
        "latex_example_rs",
        PathBuf::from("examples"),
        BuildType::PdfRustStyling,
    )
    .await
    .unwrap();
}
