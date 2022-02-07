# A Rust programming introduction in 2 hours

The goal for this repository is to provide a basic introduction to the Rust programming language, tooling, resources, and community in 2 hours. It is our hope that after completing the agenda below, readers will have the foundational knowledge and resources to continue their Rust learning journey.

## Agenda

- [Setup your development environment](#setup-your-development-environment])
- [Introduction to the Rust programming language](#introduction-to-the-rust-programming-language)
- [Programming challenges](#programming-challenges)
- [Resources](#resources)

## Setup your development environment
_20 minutes_

This section will guide you through setting up your development environment. Follow the [getting started](https://www.rust-lang.org/learn/get-started) documentation to install Rust, configure your IDE, and build a small sample application.

> NOTE: The link above will present instructions specific to your operating system.

> RECOMMENDED: For VSCode, use the following extensions:

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
- [Rust Extension Pack](https://marketplace.visualstudio.com/items?itemName=swellaby.rust-pack)

Make sure you meet Ferris when wrapping up this section! :smile:


## Introduction to the Rust programming language
_20 minutes_

This section will walk you through Rust's most [common programming concepts](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html), including [variables, mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html), [data types](https://doc.rust-lang.org/book/ch03-02-data-types.html), [functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html), [comments](https://doc.rust-lang.org/book/ch03-04-comments.html) and [control flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html).  

After completing chapter 3, review Rust's concept of [ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html), [refences and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html).

As preparation for the following programming challenges, spend a few minutes reviewing how to [store lists of values with vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)

## Programming challenges
_75 minutes_

This sections contains tutorials and programming challenges.

- [Guessing game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
  - After completing the tutorial, try adding the following features to your application:
    - If the user guesses incorrectly, inform the user of their missed guess using a _red_ font.
    - If the user guesses correctly, inform the user of their success using a _green_ font.
    - Clear the terminal when the program starts so the user has a clean terminal to start from.
    - If the user guesses the same number twice, tell them to stop repeating themselves.
    - Add scoring to keep track of the number of attempts it takes the user to guess the number.

- [Area of a triangle](https://doc.rust-lang.org/book/ch05-02-example-structs.html)
  - After completing the tutorial, try modifying your program to accept triangle dimensions from the user via console input. Don't forget to validate your input and handle any errors.

### Knowledge check
_5 minutes_

After completing the coding challengs above, see how you do with the knowledge checks below.

__Where would you go if you wanted to find crates you could import into your program?__

<details><summary>show</summary>

```
https://crates.io
```
</details>

__Where in your project would you specify dependencies on other crates, authors, and name of your binary?__

<details><summary>show</summary>

```
Cargo.toml
```
</details>

__Which file in your project folder ensures reproducible builds of your project, regardless of the the build environment?__

<details><summary>show</summary>

```
Cargo.lock
```
</details>

__(True/False) The `Cargo.lock` file should be checked into your git repository?__

<details><summary>show</summary>

__It depends!__

If you’re building a non-end product, such as a rust **library** that other rust packages will depend on, put `Cargo.lock` in your `.gitignore`. So, _False_ in this case.

If you’re building an end product, which are executable like command-line tool or an **application**, or a system library with crate-type of staticlib or cdylib, check `Cargo.lock` into git.  So, _True_ in this case.

More background on this is available [here](https://doc.rust-lang.org/cargo/faq.html#why-do-binaries-have-cargolock-in-version-control-but-not-libraries).
</details>

__What is the command to update your cargo dependencies?__

<details><summary>show</summary>

```
cargo update
```

This will result in all the dependencies specified in `Cargo.toml` to get updated to their latest version. There is also an option to tell cargo to update a specific dependency instead of all dependencies. See if you can figure out how to specify that option.
</details>

## Resources

Below are a list of resources to help you further your Rust learning journey.

- [Rust Programming Language](https://www.rust-lang.org/)
  - [The book](https://doc.rust-lang.org/book/)
  - [Rustlings course](https://github.com/rust-lang/rustlings/)
  - [Rust by example](https://doc.rust-lang.org/stable/rust-by-example/)
  - [Rust language reference](https://doc.rust-lang.org/stable/reference/)
- [Rust Package Registry](https://crates.io/)
- [Rust playground](https://play.rust-lang.org/)
- [Learning Rust](https://learning-rust.github.io/)
- [Let's Get Rusty Youtube Videos](https://www.youtube.com/c/LetsGetRusty/videos)
