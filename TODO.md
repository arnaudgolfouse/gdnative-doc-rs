- Improve link resolution
- Add a reStructuredText backend ?
- Cleanup markdown generation / find a markdown generator crate
- Find a way to generate [gut](https://github.com/bitwes/Gut) tests from documentation
- In the markdown output, move some inline links:
  ```markdown
  [mylink](url)
  ```
  to shortcut links:
  ```markdown
  [mylink]

  [mylink]: url
  ```