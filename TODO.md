- Improve link resolution
- Add a reStructuredText backend ?
- Find a way to generate [gut](https://github.com/bitwes/Gut) tests from documentation
- Add a way to refer to other user-defined methods and structures, like
  ```markdown
  [`MyStruct::my_method`]
  ```
- In the markdown output, move some inline links:
  ```markdown
  [mylink](url)
  ```
  to shortcut links:
  ```markdown
  [mylink]

  [mylink]: url
  ```
- Add a way to drive this via build script, and command line