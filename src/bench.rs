const MARKDOWN: &str = r#"
---
title: Title
author: Author
date: DD-MM-YYYY
tags: one, two, etc.
icon: devicon or fontawesome5
---

# Markdown Ipsum

## Introduction

Markdown is a lightweight markup language that allows you to format text easily. It's widely used for writing documentation, blog posts, and other forms of content. In this ipsum, we'll explore various Markdown syntax elements to showcase its flexibility and simplicity.

## Text Formatting

Let's start with some basic text formatting options:

**Bold text** can be achieved by enclosing the text within double asterisks or double underscores. For example, `**Bold text**` or `__Bold text__`.

*Italic text* can be created by enclosing the text within single asterisks or single underscores. For example, `*Italic text*` or `_Italic text_`.

***Bold and italic text*** can be achieved by combining both formatting options. For example, `***Bold and italic text***` or `___Bold and italic text___`.

## Lists

Markdown supports both ordered and unordered lists.

### Unordered List

- Item 1
- Item 2
- Item 3

### Ordered List

1. First item
2. Second item
3. Third item

## Links

Hyperlinks are straightforward in Markdown. You can create inline links like this: [Markdown Website](https://example.com/markdown). To reference a link, you can use reference-style links [1].

## Images

Embedding images is as simple as linking them. Here's an example:

![Markdown Logo](https://example.com/markdown-logo.png)

## Blockquotes

Blockquotes can be used to highlight quotes or important information:

> "Be yourself; everyone else is already taken." - Oscar Wilde

## Code

Inline code is represented by backticks. For example, to print "Hello, World!", use `println!("Hello, World!");`.

For code blocks, use triple backticks followed by the language identifier:

```rust
fn main() {
    println!("Hello, World!");
}
{}
"#;

pub fn markdown_sample(words: &str) -> String {
    MARKDOWN.replace("{}", words)
}
