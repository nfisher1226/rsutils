# RsUtils
## Introduction
These utilities are a small subset of common Unix utilities written in
the Rust programming language. The project was undertaken as an exercise
in order to gain some proficiency in Rust: nevertheless feel free to use
and share this code freely so long as the LICENSE notice is retained.
## Building
Each utility resides inside it's own subdirectory and can be built
independently. To build a single utility, cd to the corresponding
subdirectory and issue the standard "make && make install", adjusting
paths as needed using the following variables:
* PREFIX: defaults to splitting between / and /usr
* BINDIR: defaults to $PREFIX/bin and $PREFIX/sbin
* MANDIR: defaults to $PREFIX/share/man
* PROGNAME: can be adjusted to a different name, for coexistance with
the standard suite of utilities.

Alternatively, you can build the entire distribution from the top-level
directory with "make && make install", adjusting paths as above.
## Why?
There are a number of implementations of Unix base utilities available,
and there is already uutils/coreutils as a Rust implementation. This
project was undertaken primarily as an exercise to gain some experience
actually using Rust in real-world programming. However, there are a few
things that I consider lacking in one way or another.
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
* Rust crates give a rich ecosystem for adding functionality, and often
a problem that would be solved with dozens of lines of code can be
solved in Rust just by including an external crate and calling the
appropriate functions. While linking to third-party libraries is an
option in C, documentation is often lacking and portability concerns may
preclude the practice.
### GNU coreutils
* Also written in C.
* Extremely bloated codebase for what you get.
* The autotools build system is needlessly complex with a 77,874 line
configure script and a 17K plus line Makefile. To build tiny little cli
utilities that can be implemented in a single file of source code. WTF?
* The GNU folks are kind of nutty zealots, and the licensing comes with
strings attached.
* This was the only option other than Busybox for years on Linux, and
options are good.
### BusyBox
* Primarily targets embedded systems
* Extremely complex multi-call binary with a correspondingly large
attack surface, goes against the Unix way of "do one thing and do it
well".
* Another fairly complex build system, although not as complex as the
GNU system.
