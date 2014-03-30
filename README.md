rustboot
--------
A small kernel written in Rust.

It paints the screen bright red and then displays some information. You can write. That's it:

![][x86_run]

![][arm_dbg]
@adridu59 you are right. The grammar of the `use` statement was not updated in the manual ([6.1.2.1.2 Use declarations](http://static.rust-lang.org/doc/master/rust.html#use-declarations)). It's now:
```
use_decl : "pub" ? "use" [ ident '=' path
                          | path_glob ] ;
```
I propose the following:
```
use_decl : "pub" ? "use" "extern" ? [ ident '=' path
                          | path_glob ] ;
```
Even though `extern` is repeated in every `use` statement, However, 
## Setup

You need a few things to run rustboot:

1. [rust-core][rc]
2. [Rust's `master` branch][rm] or 0.9 release
3. qemu
4. On x86
  * clang
  * nasm
5. On ARM
  * binutils for arm-none-eabi
  * gcc cross-compiler
6. Optionally for debugging
  * gdb
  * tmux

Clone this repository and update rust-core.

```bash
$ git clone https://github.com/pczarn/rustboot.git
$ cd rustboot
$ git submodule update --init
### you can also pull latest rust-core:
$ git submodule foreach git pull origin master
```

To get edge Rust going, grab it from git:

```bash
$ git clone https://github.com/mozilla/rust
$ cd rust
$ ./configure
$ make && make install
```

### Arch Linux

Simply install all dependencies:
```
# pacman -S qemu nasm rust clang gdb tmux
# yaourt -S gcc-arm-none-eabi
```

### OSX

To set things up on OSX, do this:

Install `nasm` and `qemu` from homebrew:

```bash
$ brew install nasm
$ brew install qemu
```

Install latest binutils from [source][sw].

```bash
$ wget 'ftp://sourceware.org/pub/binutils/snapshots/binutils.tar.bz2'
$ ./configure --target=i386-elf
$ make && make install
```

## Running it

To compile, simply execute `make` command.

To run, use:
```bash
$ make run # emulate default platform (x86)
$ make arch=arm run   # run on ARM
$ make arch=arm debug # debug on ARM
```

[rc]: https://github.com/thestinger/rust-core
[rm]: https://github.com/mozilla/rust
[x86_run]: http://i.imgur.com/XW8PUlM.png
[arm_dbg]: http://i.imgur.com/3cHXx2D.png
[sw]: ftp://sourceware.org/pub/binutils/snapshots
