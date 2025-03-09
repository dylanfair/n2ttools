# n2ttools

This was made for the [The Elements of Computing Systems - Building a Modern Computer From First Principles](https://a.co/d/iXnqajo) book (colloquially known as [nand2tetris](https://www.nand2tetris.org/)), where readers are encouraged to make an `assembler`, `virtual machine`, and `compiler` that compiles the `Jack` programming language from the second half of book into something that runs on our `Hack` 16 bit operating system we make in the first half.

## Installation

After cloning the repo:

```sh
cargo install --path .
```

## Use

### Compiler

To compile `.jack` files into respective `.vm` files:

```sh
# for an individual file
n2ttools compile file.jack

# for a folder
n2ttools compile jack_program/

# if want to compile your current folder
n2ttools compile
# or 
n2ttools compile .
```

### Virtual Machine

To compile `.vm` files into a singular `.asm` file:

```sh
# for an individual file
n2ttools vm file.vm

# for a folder
n2ttools vm folder_of_vm_files/

# if want to compile your current folder
n2ttools vm
# or 
n2ttools vm .
```

### Assembler

To compile a singular `.asm` file into a singular `.hack` file:

```sh
n2ttools assembler file.asm
```
