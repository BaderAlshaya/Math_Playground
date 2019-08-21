# Math_Playground
Copyright (c) 2019 Bader Alshaya



## Highlights
This is a playground for several famous mathematical concepts and algorithms. The program is designed for testing, viewing, and using these concepts/algorithms digitally and maybe come up with ways to simplify them.

**Currently supported math concepts:**
- (Concept): `Perfect Numbers`
- (Concept): `Superperfect Numbers`

**Other interesting math concepts to support:**
- (Algorithm): `Sieve of Eratosthenes`



## Background Information
Below are my own interpretation of these concepts and algorithms in short.

`perfect Numbers`: A math concept that satisfies the following condition:
```
  Given a number 'n'
    'n' is a perfect number if:
      'n' == The sum of all divisors of 'n' (excluding 'n' itself)
```

`Superperfect Numbers`: A math concept that satisfies the following condition:
```
  Given a number 'n'
    'n' is a superperfect number if:
      ('n' * 2) == (The sum of all divisors of (The sum of all divisors of 'n'))
```

`Sieve of Eratosthenes`: A math algorithm to find all prime numbers within a given range. In other words:
```
  Given an upper bound 'n'
    Return all prime numbers within the range of 1 to 'n'
```



## Build and Run
Build this program and library with `cargo build`. You can
run the program with `cargo run`.
You will need to pass a
`--` before a program flag.
To build or run an optimized version, use `cargo --release`.
Use `cargo test` to run the unit tests for the current version.



## License
This program is licensed under the "MIT License". Please
see the file `LICENSE` in the source distribution of this
software for license terms.
