This example is based on https://github.com/MatejSloboda/Dijkstra_map_for_Godot.

To generate the documentation, run either
```shell
cargo build -p dijkstra-map-gd
```
Or, if you have the gdnative-doc-cli executable
```shell
gdnative-doc-cli -c config.toml --md doc/markdown --html doc/html --gut gut
```

The resulting documentation can be found in the [doc](./doc) directory, and the gut tests in the [gut](./gut) directory.