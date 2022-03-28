# brainfuck_compiler
A basic work-in progress brainfuck compiler.

## What is Brainfuck?
[Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) is an extremely simple programming language operating similarly to a turing machine.

It uses two pointers, one for instruction, and one for memory (composed of many byte size cells).

The basic operations are as follows:
| Symbol | Semantics                                                          |
|--------|--------------------------------------------------------------------|
| +      | Increment the memory cell at pointer                               |
| -      | Decrement the memory cell at pointer                               |
| >      | Move the pointer one cell to the left                              |
| <      | Move the pointer 1 cell to the right                               |
| .      | Output the value of the cell at pointer                            |
| ,      | Input a value to the cell at pointer                               |
| [      | If the cell at pointer is zero, jump to after the next ']'         |
| ]      | If the cell at pointer is non-zero, jump to after the previous '[' |

Typically 30,000 memory cells are available, through this differs based on implementation. In my compiler the use can set both the start position and the 
number of cells.

I have added two more symbols to the grammar:
| Symbol | Semantics                                                                                                                         |
|--------|-----------------------------------------------------------------------------------------------------------------------------------|
| #      | Delimits comments                                                                                                                 |
| ::     | Delimits inserts (to be placed in the same position relative to compiled brainfuck in the compiled result) (e.g assembly inserts) |

## Features
Basic features of the project are:
- Can transpile brainfuck to C [Working]
- Can compile brainfuck to ARM [Developing]
- Can interpret the brainfuck IR directly [Developing]
- Printouts for all representations (for education)
- Architecture neutral optimisations on the brainfuck IR, using peephole optimisations and pattern matching.

There are also some extensions to the brainfuck language:
- Comments (delimited by `#`)
- Inserts (delimited by `::`) allow users to write any arbitrary string into the compiled brainfuck at a given position in the brainfuck program. 
  This allows for complex functionality to be written in the target language (e.g assembly inserts required for building a BrainFuck OS)

## Usage
Once the repo is cloned, simply use the provided makefile
```
make # Produces the bfc binary in the repo toplevel directory
```
To run the compiler and view the help menu with all options simply use
```
./bfc -h
```

For example:
```
>> make
>> echo "#this is an example program#,>,[<->-]<++++++++++++++++++++++++++++++++++++++++++++++++." > add_two_numbers.bf
>> ./bfc add_two_numbers.bf -t c99 && gcc add_two_numbers.c && a.out
>> da
3
```
For example the above program computes difference between the first and second character, printing it (prints correctly if 0-9)

## Design
![v1 compile map](https://user-images.githubusercontent.com/44177991/160304858-15c1ecf2-caf2-40c9-9fdb-9342696f82b7.png)
### Parser
I am using [nom](https://docs.rs/nom/latest/nom/) for a basic parser. Brainfuck's simple grammar helps in this respect.

### Intermediate Representation
To allow for architecture-neutral optimisations, the IR supports pointer move operations of any size, as well as operations to multiply, modulus, divide, and add any arbitrary integer by the current memory cell.

Hence many brainfuck operations can be combined. (e.g `+++` becomes `+(3)` and `>>><` `>(2)`)

### Architecture Specific Optimisations
[Planned] Many patterns in brainfuck can be replaced with more optimal assembly inserts.
```
{n, ..., m} => {n + m,..., 0} (pointer at second cell)
[<(...)+>(...)-]
```
can be converted into:
```assembly
@ given r0 has the memory pointer, x is the distance between the cells being added
ldr r1, [r0]
ldr r2,[r0, #-x]
add r1, r1, r2
str r1, [r0]
mov r1, #0
str r1, [r0, #-x]
```
Once most common patterns have been replaced with more optimal inserts, all code can be translated.