# Articles

Articles are built to cover different topics related to a language, 
this could be news, tutorials, or anything else that is relevant to the language.
To properly organize the different articles in our repository, we use a specification 
which will regularly update to keep up with the needs of the community.

## Article Format
The general idea is to create a markdown file which contains a metadata section at the top delimited by `---` and 
after that, the content of the article. The metadata section is used to provide information about the article,
and is used to display the article in the website.

### Metadata
The metadata section is a YAML document which contains the following fields:
- `title`: The title of the article.
- `description`: A short description of the article.
- `author`: The author of the article.
- `date`: The date the article was published (YYYY-MM-DD).
- `tags`: A list of tags for the article.
- `icon`: The icon to use for the article (`devicon` or `fontawesome5`).

Inside of Markdown it will look like this:
```markdown
---
title: My Article
description: This is my article
author: John Doe
date: 2020-01-01
tags: 
    - tag1
    - tag2
icon: devicon
---

Content of the article
```