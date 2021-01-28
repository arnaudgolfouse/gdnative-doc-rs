# gdnative-doc-cli

By executing `cargo build --release`, you should get an executable at `target/release/gdnative-doc-cli`. Then, to test it on a rust crate:
```
cd <path-to-my-crate>
gdnative-doc-cli --md <path-to-markdown-output>
```

To get more options, run `gdnative-doc-cli --help`.