# How To Use Rust

[![Language](https://img.shields.io/badge/Language-Rust-informational.svg)](https://github.com/abesuden/sandbox/Cython)
[![Compiler](https://img.shields.io/badge/Compiler-cargo-1abc9c.svg)](https://github.com/abesuden/sandbox/Cython)

<p align="center">
    
[![Rust Logo](https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/1024px-Rust_programming_language_black_logo.svg.png)

</p>


This is a guide on how to setup and use Rust in WSL with VS Code.

## To Install 

Go to the Rust website [here](https://www.rust-lang.org/tools/install).

> once installed, restart the WSL environment

## Using Rust

If you can use `rustc --version`, then the environment is setup and recognized. 
### VS Code

I recomend searching for the "Rust (rls)" extension by rust-lang.

### Windows Subsystem Linux (WSL) in VS Code

Remoting into WSL with VS Code seems to have an issue with the PATH. Thus, you will need to use this command every time you start up a new WSL environment in VS Code.

```
source $HOME/.cargo/env
```

I extended the terminal to make this easier:

<details>
<summary><strong>How To Extend Terminal Guide</strong> (click to expand)</summary>
 1) `cd /bin/`
 1) `sudo nano rust`
 1) place the code into the file from below
 1) ctrl + o (to save)
 1) ctrl + x (to exit)
 1) `sudo chmod 755 rust`
 1) `rust`
</details>

The bash script that the command `rust` will run:

```
#!/bin/bash

source $HOME/.cargo/env
rustc --version
```

> If the command works, then the version of rust is printed out and you are good to go!

## Create Projects

Create a new folder:

```
cargo new folderName
```

Create a new project inside the current folder:

```
cargo init
```

## Folder Structure

Notice that there is a `.toml` file type. It has package and dependency information about the project (similar to a pip file for python or a package.json for nodeJS).
> you can install "Better TOML" to add colors

## Compile Rust Programs

When in this workflow, you will use cargo to compile and run with the following command:

```
cargo run
```
> it compiles into the target `folder/debug`

If you just want to build the project, use the following command:

```
cargo build
```

If you want to build for production, use:

```
cargo build --release
```
> this optimizes and versions the project build. The executable can be found under the `/release` directory path.

Otherwise, the following commands can be used without using cargo:

```
rustc fileName.rs
```

This command creates an executable file that can be run with the following command:

```
./fileName
```
> similar to how C executables are executed

## Rust Language

Things to know about programming with Rust

### Rules/Features

- In general, Rust tends to use CamelCase for "type-level" constructs (types and traits) and snake_case for "value-level" constructs. Find Rust documentation on this [here](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html).
- Variables are immutable by default
- variables hold primitive data or references to data
- Rust is a block-scoped language (variables in funcitons are scoped to that funciton)
- Statically typed language
    * The compiler needs to know all types of variables
    * However, the compiler can infer based on assigned type
- Tuples have a max size of 12 elements