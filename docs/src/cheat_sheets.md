# Cheat Sheets

Cheat sheets are a great way to quickly learn something new. They are also a great way to refresh your memory on something you've already learned. 
In our website, we seperate the cheat sheets by language, and then by level of difficulty. This allows for users to see the 
beginner, intermediate, and advanced cheat sheets for a language.

## Cheat Sheet Format
The general idea is to create a markdown file which contains a metadata section at the top delimited by `---` and
after that, the content of the cheat sheet. The metadata section is used to provide information about the cheat sheet,
and is used to display the cheat sheet in the website.

### Metadata
The metadata section is a YAML document which contains the following fields:
- `title`: The title of the cheat sheet.
- `author`: The author of the cheat sheet.
- `level`: The level of the cheat sheet (1, 2, or 3).
  - 1: Beginner
  - 2: Intermediate
  - 3: Advanced
- `language`: The language of the cheat sheet.
- `icon`: The icon to use for the cheat sheet (`devicon` or `fontawesome5`).

Inside of Markdown it will look like this:
```markdown
---
title: My Cheat Sheet
author: John Doe
level: 1
language: python
icon: devicon
---
Content of the cheat sheet
```