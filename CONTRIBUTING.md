Hi, and thank you for your interest 😁 !

# Contribute

Making a contribution follows the common github workflow:
- [Fork](https://docs.github.com/en/github/getting-started-with-github/fork-a-repo#fork-an-example-repository) this repository, and clone your fork locally.
- Create a new git branch (using e.g. `git switch -c new-branch`).
- Hack, code, modify... Try to follow the [Code style](#Code-style)
- Push your branch to your fork.
- Create a [pull request](https://docs.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request-from-a-fork).

I am the sole reviewer at the moment, so it might take some time until I merge your PR 🙂

# Code style

- ## Cargo fmt
    Keep the files formatted using `cargo fmt`.

- ## Tests

    - You should preferably run the tests with `cargo test` before submitting 
your pull request to avoid surprises.
    - If you are adding a feature, try to add a corresponding test when possible.
    - Especially if your contribution changes the markdown output, you should 
adjust/add tests accordingly.
    
	Markdown-oriented tests are located in `src/backend/markdown/tests.rs`, and use [insta](https://insta.rs/) to test against reference values.

- ## Examples

    If your contribution changes the markdown/html/gut output, it would be 
	good to update the example in `example/dijkstra-map-gd`.

	To do so, simply go to `example/dijkstra-map-gd` and run `cargo build`.