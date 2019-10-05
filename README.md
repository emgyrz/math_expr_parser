# Math expressions parser

Just for training

Dijkstra’s algorithm used


![alt text](example.gif)


### Run
```sh
cargo run -- "5.23 + 41 * 0.79"
cargo run -- "(1 + 2) * -((3 + 4) - 9)"
```


#### Accepts
* floats
* simple math operations `+` | `-` | `*` | `/`
* round brackets `(` | `)`
* negative numbers and expressions in brackets `-9.1`, `-(5 + 2)`
* exponentiation `^`



