# ansi-colors
![[CircleCI](https://circleci.com/gh/l-tools/ansi-colors.svg?style=svg)](https://circleci.com/gh/l-tools/ansi-colors)  ![crates.io](https://img.shields.io/crates/v/ansi-colors.svg) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) ![doc.rs](https://docs.rs/ansi-colors/badge.svg?version=0.1.0)
## What is it and what is it for?

ansi-colors is a rust crate(library) that should be used to format terminal string outputs.
ansi-colors offers the ansi string coloring pallet, and the string formatting pallet in order to help you color, format and beutify you output!
ansi-colors works best(for now) with the ubuntu terminal coloring scheme, but it still may work on some windows and mac computers.

## How to install it?

### you can use cargo(the preferable and much easier and safe way):
```console
:~$ cargo install ansi-colors
```
### or you can clone this repository:
```console
:~$ git clone https://github.com/l-tools/ansi-colors ansi-colors
:~$ cd ansi-colors
:~$ cargo build
```

## Usage

Add the following to your Cargo.toml:
```rust
[dependencies]
ansi-colors = "0.1.0"
```
First use import the crate:
```rust
extern crate ansi_colors;  
use ansi_colors::*; 
```

Then you create a new mutable coloured string using an &str primitive type
```rust
fn main(){                                                             
	let mut str1 = ColouredStr("hello ansi");
```
Next comes the formatting:
```rust
	str1.blue();
	str1.bold();
	str1.underline();
```
Finally you print:
```rust
	println!("{}",str1);
}
```
Together it looks like that:
```rust
fn main(){                                                             
	let mut str1 = ColouredStr("hello ansi");
	str1.blue();
	str1.bold();
	str1.underline();
	println!("{}",str1);
}
```
Or this:
```rust
fn main(){                                                            
	let mut str1 = ColouredStr("ERROR!!!!!");
	str1.red();
	str1.bold();
	str1.blink();
	str1.underline();
	println!("{}",str1);
}
```
### If you want to checkout the crate further that you should take a look in the [examples](https://github.com/l-tools/ansi-colors/tree/master/examples) folder.
## Currently being developed:
1) background and reseting options.
2) the ability to use them in the same row(str1.blue().bold() for example).
3) integrating with termi-graphics(my other crate), and creating some higher level api for the terminal graphics.

# License 
This crate is primarily distributed under the terms of the MIT license
See  [LICENSE-MIT](https://github.com/l-tools/ansi-colors/blob/master/LICENSE) for details.
