# RsUtils
## Introduction
These utilities are a small subset of common Unix utilities written in
the Rust programming language. The project was undertaken as an exercise
in order to gain some proficiency in Rust: nevertheless feel free to use
and share this code freely so long as the LICENSE notice is retained.
## Building
Each utility resides inside it's own subdirectory and can be built
independently. To build a single utility, cd to the corresponding
subdirectory and issue the standard `make && make install`, adjusting
paths as needed using the following variables:
* PREFIX: defaults to splitting between / and /usr
* BINDIR: defaults to $PREFIX/bin and $PREFIX/sbin
* MANDIR: defaults to $PREFIX/share/man
* PROGNAME: can be adjusted to a different name, for coexistance with
the standard suite of utilities.

Alternatively, you can build the entire distribution from the top-level
directory with `make && make install`, adjusting paths as above.

Alternative to the alternative, each utility can be built with `cargo
build`, which is what the Makefiles do internally.
## Why Rust?
Rust is comparable to C in the control it give the programmer to manage
memory directly, while giving added peace of mind that memory and
concurrency safety are baked right in to the language at compile time.
Generally if the Rust compiler compiles the code, entire classes of
common bugs and security issues that are common in C and C++ are not
going to be present.

Rust crates give a rich ecosystem for adding functionality, and often
a problem that would be solved with dozens of lines of code can be
solved in Rust just by including an external crate and calling the
appropriate functions. While linking to third-party libraries is an
option in C, documentation is often lacking and portability concerns may
preclude the practice.
## Why another implementation?
There are a number of implementations of Unix base utilities available,
and there is already uutils/coreutils as a Rust implementation. This
project was undertaken primarily as an exercise to gain some experience
actually using Rust in real-world programming. However, there are a few
things that I consider lacking in the other implementations in  one way
or another.
### uutils/coreutils
* uutils/coreutils is designed to be cross platform, leading to some
code bloat and tradeoffs in implementation details. RsUtils by contrast
targets Posix-ey systems only, and aims for correctness and light
weight.
* uutils uses quite a bit of shared code between utilities and by
default builds a single multi-call binary. RsUtils are all completely
standalone, allowing one to pick and choose what gets installed.
* Certain implementation details are non-standard in uutils/coreutils,
such as mkdir, which according to Posix should accept all forms of mode
argument that chmod accepts, but which only accepts permission modes in
octal format.
### sbase, lobase
* Written in C, not Rust. While it is completely possible to write good
code in C, Rust enforces good code by default in respect to memory
safety and concurrency.
### GNU coreutils
* Also written in C.
* Extremely bloated codebase for what you get.
* The autotools build system is needlessly complex with a 77,874 line
configure script and a 17K plus line Makefile. To build tiny little cli
utilities that can be implemented in a single file of source code. I do
not consider this to be good practice.
* The GNU folks are kind of nutty zealots, and the licensing comes with
strings attached.
* This was the only option other than Busybox for years on Linux, and
options are good.
### BusyBox
* Primarily targets embedded systems
* Extremely complex multi-call binary with a correspondingly large
attack surface, goes against the Unix way of "do one thing and do it
well".
* For complete functionality of all applets, must be install suid.
* Another fairly complex build system, although not as complex as the
GNU system.
