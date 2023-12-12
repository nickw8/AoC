# Advent of Code 2023

This is my first year attempting this and decided to do it in Rust cause why not make it harder on myself. I have been wanting to learn Rust better and going through these, knowing how I would do it but figuring out how to do it in Rust idiomatically will be a lot of fun and a lot of learning.

Saw one of the guys on Youtube talking about their setup for AoC and automating it using [just](https://github.com/casey/just) and that looked easier than just copy and pasting each day and deleting the code. Following his example I will use the following commands:
- `just create` and pass it a day (`day-01`) which will create a new day based on the template in `daily-template`
- `just work` and pass it a day (`day-01`) and the part (`part1`) I am working on and this will run `cargo-watch` and `nexttest` on that part.

## Prepare for a new day

```shell
just create <day>
```

## [Just](https://github.com/casey/just)

```shell
brew install just
```

## cargo-nextest

> [cargo-nextest][cargo-nextest] is "a next-generation test runner for Rust projects". Basically that means it includes [an interesting execution model][cargo-nextest-execution-model] than can be great for projects with a _lot_ of tests.
>
>As of this year's AoC, [cargo-nextest][cargo-nextest] doesn't run doctests yet, so while that won't be an issue for us it is something to be aware of if you're using nextest in a "real project". (Basically that means you also run `cargo test --doc`).
>
>cargo-nextest has what I consider [a positive relationship with the regular `cargo test`](https://nexte.st/book/how-it-works.html#contributing-features-back-to-cargo) and is rightfully a nice place to be experimenting with new testing UX. `cargo test` works well and `cargo nextest` is a forward-looking place for experimentation.
>
> [Christopher Biscardi](https://github.com/ChristopherBiscardi/advent-of-code/blob/main/2023/rust/README.md)

```shell
cargo install cargo-nextest
```

[just]https://github.com/casey/just
[cargo-nextest]: https://nexte.st/
[cargo-nextest-execution-model]: https://nexte.st/book/how-it-works.html
