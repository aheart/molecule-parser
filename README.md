# molecule-parser
Molecule parser based on CodeWars Kata https://www.codewars.com/kata/molecule-to-atoms/train/rust


## Usage example
```
$ cargo run "K4[ON(SO3)2]2"
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/molecule_parser 'K4[ON(SO3)2]2'`
Atoms: Ok([("K", 4), ("O", 14), ("N", 2), ("S", 4)])
```


## Running tests
```
$ cargo test
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/molecule_parser-acd46ce6a5cf87b0

running 6 tests
test parser::test::fremys_salt ... ok
test parser::test::hydrogen ... ok
test parser::test::magnesium_hydroxide ... ok
test parser::test::oxygen ... ok
test parser::test::test_fails ... ok
test parser::test::water ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Inspired by https://adriann.github.io/rust_parser.html
