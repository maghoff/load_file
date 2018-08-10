[![Build Status](https://travis-ci.org/maghoff/load_file.svg?branch=master)](https://travis-ci.org/maghoff/load_file)
[![Latest version](https://img.shields.io/crates/v/load_file.svg)](https://crates.io/crates/load_file)
[![Documentation](https://docs.rs/load_file/badge.svg)](https://docs.rs/load_file/)

This crate provides macros to help conveniently load the contents of
files during development.

`load_str!` and `load_bytes!` are modeled after `include_str!` and
`include_bytes!` from the standard library. The standard library macros
are useful in many situations, one of which is quick-and-dirty loading of
assets during a prototyping phase. (Examples of such assets could be
static web assets such as CSS or GLSL shaders for a game.) The `load_*`
macros aim to offer a convenient stepping stone to move from this to
loading the assets dynamically at run-time. This gets rid of the need to
compile or even restart for every change while iterating on the assets.

# Example
Before:

    fn main() {
        println!("{}", include_str!("greeting.txt"));
    }

After:

    #[macro_use]
    extern crate load_file;

    fn main() {
        println!("{}", load_str!("greeting.txt"));
    }
