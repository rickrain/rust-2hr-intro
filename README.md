# A Rust programming introduction in 2 hours

The goal for this repo is to provide a basic introduction to the Rust programming language, tooling, resources, and community in 2 hours. The reader should be able to leverage this knowledge to further develop their Rust programming skills.

## Agenda

- [Setup your development environment](#setup-your-development-environment])
- [Introduction to the Rust programming language](#introduction-to-the-rust-programming-language)
- [Programming challenges](#programming-challenges)
- [Resources](#resources)

## Setup your development environment

This section will guide you through setting a development environment on [Ubuntu Desktop](#ubuntu-desktop), [Mac](#mac), or [Windows](#windows). If you don't want to setup an environment, there are [alternatives](#alternatives) to get you coding in Rust quickly.
### Ubuntu Desktop


> These instructions have were tested on Ubuntu Desktop 21.10.

TODO...

### Mac

TODO...
### Windows

TODO...
### Alternative

Do you just want to kick the tires in Rust witout setting up an environment? If so, try out the [Rust playground](https://play.rust-lang.org/), where you can experiment with the language online without setting up an environment.

## Introduction to the Rust programming language

TODO...

## Programming challenges

This sections contains tutorials and programming challenges.

- [Guessing game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
  - After completing the tutorial, try adding the following features to your application:
    - If the user guesses incorrectly, inform the user of their missed guess using a _red_ font. Also, consider sounding an abnoxious horn if the user guesses incorrectly.
    - If the user guesses correctly, inform the user of their success using a _green_ font. Also, consider sounding a pleasant bell chime if the user guess correctly.
    - Clear the terminal when the program starts so the user has a clean terminal to start from.
    - If the user guesses the same number twice, tell them to stop repeating themselves.

- [Area of a triangle](https://doc.rust-lang.org/book/ch05-02-example-structs.html)
  - After completing the tutorial, try modifying your program to accept triangle dimensions from the user via console input. Don't forget to validate your input and handle any errors.

### Knowledge check

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
cargo.toml
```
</details>

__Which file in your project folder ensures reproducible builds of your project, regardless of the the build environment?__

<details><summary>show</summary>

```
cargo.lock
```
</details>

__(True/False) The `cargo.lock` file should be committed to your git repository?__

<details><summary>show</summary>

```
True: This file specifies the specific dependency versions of other crates your project depends on. So, to ensure consistent builds, even in an automated environment, this file should be part of your git repository.
```
</details>

__What is the command to update your cargo dependencies?__

<details><summary>show</summary>

```
cargo update

This will result in all the dependencies specified in cargo.toml to get updated to their latest version. There is also an option to tell cargo to update a specific dependency instead of all dependencies. See if you can figure out how to specify that option.
```
</details>

## Resources

Below are a list of resources to help you further your Rust learning journey.

- [Rust Programming Language](https://www.rust-lang.org/)
- [Rust Package Registry](https://crates.io/)
- [Let's Get Rusty Youtube Videos](https://www.youtube.com/c/LetsGetRusty/videos)
